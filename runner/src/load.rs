//! Action + checks: read `graphs/*.json` into [`Graph`] values.
//!
//! [`check`] mirrors the shape rules of `scripts/verify-graphs`. It does not
//! replace it: `verify-graphs` is the gate, this loader is a consumer
//! (decision 7). Persona and skill *existence* are not checked here — that is
//! `verify-skill-refs`' job and needs the catalog.

use std::collections::{BTreeMap, HashSet};
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::graph::{Graph, Label, NodeId};
use crate::step::{self, Resolve};

/// The schema file, skipped by [`load_dir`].
pub const SCHEMA_FILE: &str = "schema.json";

/// Every graph of one directory, keyed by file stem — the name a
/// `uses.graph` refers to. Sorted, so iteration is file-name order.
pub type Graphs = BTreeMap<String, Graph>;

impl Resolve for Graphs {
    fn resolve(&self, name: &str) -> Option<&Graph> {
        self.get(name)
    }
}

/// Which end of an edge failed to resolve.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeEnd {
    /// `edges[].from`
    From,
    /// `edges[].to`
    To,
}

impl fmt::Display for EdgeEnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            EdgeEnd::From => "from",
            EdgeEnd::To => "to",
        })
    }
}

/// Why a graph file was refused. Every variant names the graph and, where
/// one is involved, the node or edge.
#[derive(Debug, Error)]
pub enum LoadError {
    /// The file could not be read.
    #[error("{}: {source}", path.display())]
    Io {
        /// Path that failed.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },
    /// Not JSON, or JSON the schema refuses (unknown key, unknown role,
    /// wrong type, missing required key). The message names the key.
    #[error("{graph}: {source}")]
    Json {
        /// Graph name (file stem).
        graph: String,
        /// Underlying serde error.
        #[source]
        source: serde_json::Error,
    },
    /// `apiVersion` is not [`Graph::API_VERSION`].
    #[error(
        "{graph}: apiVersion must be '{}', found '{found}'",
        Graph::API_VERSION
    )]
    ApiVersion {
        /// Graph name.
        graph: String,
        /// Value found in the file.
        found: String,
    },
    /// `kind` is not [`Graph::KIND`].
    #[error("{graph}: kind must be '{}', found '{found}'", Graph::KIND)]
    Kind {
        /// Graph name.
        graph: String,
        /// Value found in the file.
        found: String,
    },
    /// `nodes` is empty.
    #[error("{graph}: nodes must be a non-empty list")]
    NoNodes {
        /// Graph name.
        graph: String,
    },
    /// Two nodes share an id.
    #[error("{graph}: duplicate node id '{node}'")]
    DuplicateNode {
        /// Graph name.
        graph: String,
        /// The repeated id.
        node: NodeId,
    },
    /// `start` names no node.
    #[error("{graph}: start '{start}' is not a node")]
    StartNotANode {
        /// Graph name.
        graph: String,
        /// The dangling start id.
        start: NodeId,
    },
    /// An edge endpoint names no node.
    #[error("{graph}: edge {index} {end} '{node}' is not a node")]
    DanglingEdge {
        /// Graph name.
        graph: String,
        /// Position in `edges`.
        index: usize,
        /// Which end dangles.
        end: EdgeEnd,
        /// The dangling id.
        node: NodeId,
    },
    /// Two out-edges of one node carry the same `when` (or are both
    /// unlabeled), so no event could pick one.
    #[error("{graph}: node '{node}' has two out-edges {}", describe_when(.label.as_ref()))]
    AmbiguousOutEdges {
        /// Graph name.
        graph: String,
        /// The node with the ambiguous out-edges.
        node: NodeId,
        /// The repeated label; `None` when both edges are unlabeled.
        label: Option<Label>,
    },
    /// An edge carries `when: "next"`, which is reserved for unlabeled edges.
    #[error(
        "{graph}: edge {index} {from} -> {to} uses the reserved label '{}'",
        Label::NEXT
    )]
    ReservedLabel {
        /// Graph name.
        graph: String,
        /// Position in `edges`.
        index: usize,
        /// Source node id.
        from: NodeId,
        /// Target node id.
        to: NodeId,
    },
    /// A node sets both `uses.graph` and `uses.skill`.
    #[error("{graph}: node '{node}' uses.graph is exclusive of uses.skill")]
    ExclusiveUses {
        /// Graph name.
        graph: String,
        /// The offending node.
        node: NodeId,
    },
    /// A node's `uses.graph` names the graph it lives in.
    #[error("{graph}: node '{node}' must not reference its own graph")]
    SelfReference {
        /// Graph name.
        graph: String,
        /// The offending node.
        node: NodeId,
    },
    /// The directory holds no graph JSON files.
    #[error("{}: no graph JSON files", dir.display())]
    NoGraphs {
        /// Directory searched.
        dir: PathBuf,
    },
    /// A `role: graph` node's `uses.graph` names a file the directory does
    /// not hold, so no walk could descend into it.
    #[error("{graph}: node '{node}' uses.graph '{sub}' has no {sub}.json in the directory")]
    MissingSubgraph {
        /// Graph name.
        graph: String,
        /// The `role: graph` node.
        node: NodeId,
        /// The `uses.graph` value.
        sub: String,
    },
}

fn describe_when(label: Option<&Label>) -> String {
    match label {
        Some(l) => format!("with when '{l}'"),
        None => format!("without when (both fire on '{}')", Label::NEXT),
    }
}

/// Check a parsed graph against the rules `schema.json` cannot express:
/// `apiVersion` / `kind` constants, non-empty nodes, unique ids, a `start`
/// that is a node, edge endpoints that are nodes, unambiguous out-edges, no
/// reserved `next` label, `uses.graph` exclusive of `uses.skill`, no
/// self-reference.
///
/// # Errors
///
/// The first rule violated, as a [`LoadError`] naming the graph and the
/// node or edge.
pub fn check(g: &Graph) -> Result<(), LoadError> {
    let graph = || g.name.clone();
    if g.api_version != Graph::API_VERSION {
        return Err(LoadError::ApiVersion {
            graph: graph(),
            found: g.api_version.clone(),
        });
    }
    if g.kind != Graph::KIND {
        return Err(LoadError::Kind {
            graph: graph(),
            found: g.kind.clone(),
        });
    }
    if g.nodes.is_empty() {
        return Err(LoadError::NoNodes { graph: graph() });
    }
    let mut ids: HashSet<&NodeId> = HashSet::with_capacity(g.nodes.len());
    for n in &g.nodes {
        if !ids.insert(&n.id) {
            return Err(LoadError::DuplicateNode {
                graph: graph(),
                node: n.id.clone(),
            });
        }
        if let Some(sub) = n.uses.as_ref().and_then(|u| u.graph.as_ref()) {
            if n.uses.as_ref().is_some_and(|u| u.skill.is_some()) {
                return Err(LoadError::ExclusiveUses {
                    graph: graph(),
                    node: n.id.clone(),
                });
            }
            if *sub == g.name {
                return Err(LoadError::SelfReference {
                    graph: graph(),
                    node: n.id.clone(),
                });
            }
        }
    }
    if !ids.contains(&g.start) {
        return Err(LoadError::StartNotANode {
            graph: graph(),
            start: g.start.clone(),
        });
    }
    let mut seen: HashSet<(&NodeId, Option<&Label>)> = HashSet::with_capacity(g.edges.len());
    for (index, e) in g.edges.iter().enumerate() {
        for (end, node) in [(EdgeEnd::From, &e.from), (EdgeEnd::To, &e.to)] {
            if !ids.contains(node) {
                return Err(LoadError::DanglingEdge {
                    graph: graph(),
                    index,
                    end,
                    node: node.clone(),
                });
            }
        }
        if e.when.as_ref().is_some_and(Label::is_reserved) {
            return Err(LoadError::ReservedLabel {
                graph: graph(),
                index,
                from: e.from.clone(),
                to: e.to.clone(),
            });
        }
        if !seen.insert((&e.from, e.when.as_ref())) {
            return Err(LoadError::AmbiguousOutEdges {
                graph: graph(),
                node: e.from.clone(),
                label: e.when.clone(),
            });
        }
    }
    Ok(())
}

/// Parse `text` as the graph named `graph` (the file stem; used in the error
/// when the JSON never yields a name) and [`check`] it.
///
/// # Errors
///
/// [`LoadError::Json`] when the text is not a schema-shaped graph; otherwise
/// whatever [`check`] refuses.
pub fn parse(graph: &str, text: &str) -> Result<Graph, LoadError> {
    let g: Graph = serde_json::from_str(text).map_err(|source| LoadError::Json {
        graph: graph.to_owned(),
        source,
    })?;
    check(&g)?;
    Ok(g)
}

/// Read one graph file.
///
/// # Errors
///
/// [`LoadError::Io`] when the file cannot be read; otherwise as [`parse`].
pub fn load_file(path: &Path) -> Result<Graph, LoadError> {
    let text = fs::read_to_string(path).map_err(|source| LoadError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    parse(&stem_of(path), &text)
}

/// Check that every `role: graph` node's `uses.graph` is a key of `graphs`,
/// so every descent in the set has somewhere to go.
///
/// # Errors
///
/// [`LoadError::MissingSubgraph`] for the first `role: graph` node, in
/// file-name then document order, whose subgraph is not in the set.
pub fn check_subgraphs(graphs: &Graphs) -> Result<(), LoadError> {
    for g in graphs.values() {
        for n in &g.nodes {
            if let Some(sub) = step::subgraph_of(g, &n.id) {
                if !graphs.contains_key(sub) {
                    return Err(LoadError::MissingSubgraph {
                        graph: g.name.clone(),
                        node: n.id.clone(),
                        sub: sub.to_owned(),
                    });
                }
            }
        }
    }
    Ok(())
}

/// Read every `*.json` in `dir` except [`SCHEMA_FILE`] into a [`Graphs`]
/// keyed by file stem, then check that every `role: graph` node names a
/// graph in the set, so the result is a complete [`Resolve`].
///
/// # Errors
///
/// [`LoadError::Io`] when the directory cannot be listed,
/// [`LoadError::NoGraphs`] when it holds no graph files, the first file
/// (in file-name order) that [`load_file`] refuses, or
/// [`LoadError::MissingSubgraph`] when a descent has nowhere to go.
pub fn load_dir(dir: &Path) -> Result<Graphs, LoadError> {
    let io = |source| LoadError::Io {
        path: dir.to_path_buf(),
        source,
    };
    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(dir).map_err(io)? {
        let path = entry.map_err(io)?.path();
        let is_json = path.extension().and_then(OsStr::to_str) == Some("json");
        let is_schema = path.file_name().and_then(OsStr::to_str) == Some(SCHEMA_FILE);
        if is_json && !is_schema {
            paths.push(path);
        }
    }
    if paths.is_empty() {
        return Err(LoadError::NoGraphs {
            dir: dir.to_path_buf(),
        });
    }
    paths.sort();
    let mut graphs = Graphs::new();
    for path in &paths {
        let g = load_file(path)?;
        graphs.insert(stem_of(path), g);
    }
    check_subgraphs(&graphs)?;
    Ok(graphs)
}

/// The file stem, or the whole path when it has none.
fn stem_of(path: &Path) -> String {
    path.file_stem()
        .and_then(OsStr::to_str)
        .map_or_else(|| path.display().to_string(), str::to_owned)
}

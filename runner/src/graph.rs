//! Data: the typed reading of one `graphs/*.json` file.
//!
//! These types encode `graphs/schema.json`; they do not re-invent it. Every
//! struct is `deny_unknown_fields`, so a key the schema refuses is refused
//! here too. No checks live in this module — see [`crate::load`].

use std::fmt;

use serde::{Deserialize, Serialize};

/// A node identifier (`nodes[].id`, `start`, `edges[].from` / `to`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(transparent)]
pub struct NodeId(String);

impl NodeId {
    /// Wrap a node id.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// The id as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// An edge label (`edges[].when`).
///
/// The graphs' own vocabulary: `BLESS`, `REJECT`, `pass`, `human`, ...
/// [`Label::NEXT`] is reserved for unlabeled edges and may not appear in a
/// file (decision 3).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(transparent)]
pub struct Label(String);

impl Label {
    /// The event that fires an unlabeled edge. Reserved: a `when: "next"`
    /// in a graph file fails to load.
    pub const NEXT: &'static str = "next";

    /// Wrap a label.
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self(label.into())
    }

    /// The label as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// `true` when this label is the reserved [`Label::NEXT`].
    #[must_use]
    pub fn is_reserved(&self) -> bool {
        self.0 == Self::NEXT
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// `nodes[].role`. Mirrors `schema.json`
/// `properties.nodes.items.properties.role.enum`, in order.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum Role {
    Generator,
    Adversary,
    Stage,
    Gate,
    Terminal,
    Graph,
}

impl Role {
    /// Every role, in schema order. A test compares this to the schema's
    /// enum so the two cannot drift silently (decision 7).
    pub const ALL: [Role; 6] = [
        Role::Generator,
        Role::Adversary,
        Role::Stage,
        Role::Gate,
        Role::Terminal,
        Role::Graph,
    ];

    /// The role as the schema spells it (kebab-case).
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Role::Generator => "generator",
            Role::Adversary => "adversary",
            Role::Stage => "stage",
            Role::Gate => "gate",
            Role::Terminal => "terminal",
            Role::Graph => "graph",
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// `nodes[].uses`: what a node loads. All optional; `graph` is exclusive of
/// `skill` (checked at load time).
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Uses {
    /// Skill name. Owned by crossr-skills when the node is `catalog: true`.
    pub skill: Option<String>,
    /// Persona file name under `.agents/agents/`.
    pub persona: Option<String>,
    /// Subgraph name: `graphs/<graph>.json`.
    pub graph: Option<String>,
}

/// One entry of `nodes`.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Node {
    /// Unique within the graph.
    pub id: NodeId,
    /// Optional role.
    pub role: Option<Role>,
    /// `uses.skill` is owned by crossr-skills.
    pub catalog: Option<bool>,
    /// Adversary reviews a set; one verdict line per id.
    pub batch: Option<bool>,
    /// What the node loads.
    pub uses: Option<Uses>,
}

/// One entry of `edges`. An edge without `when` fires on [`Label::NEXT`].
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Edge {
    /// Source node id.
    pub from: NodeId,
    /// Target node id.
    pub to: NodeId,
    /// Event label; `None` means unlabeled.
    pub when: Option<Label>,
}

/// Top-level `requires`.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Requires {
    /// Catalog skills the graph names.
    pub skills: Option<Vec<String>>,
    /// Runs only against a disclosed language book; the graph never names one.
    pub book: Option<bool>,
}

/// One `graphs/<name>.json`.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Graph {
    /// Must equal [`Graph::API_VERSION`] (checked after parse).
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Must equal [`Graph::KIND`] (checked after parse).
    pub kind: String,
    /// Graph name; the file stem.
    pub name: String,
    /// Entry node id. The runner starts here; document order is not the rule.
    pub start: NodeId,
    /// Human title.
    pub title: Option<String>,
    /// Human description.
    pub description: Option<String>,
    /// Local conductor skill that remains the law for this graph. A name
    /// only; the runner never reads the skill file.
    pub conductor: Option<String>,
    /// Nodes, in document order.
    pub nodes: Vec<Node>,
    /// Edges, in document order.
    pub edges: Vec<Edge>,
    /// Catalog requirements.
    pub requires: Option<Requires>,
}

impl Graph {
    /// The only `apiVersion` this reader accepts.
    pub const API_VERSION: &'static str = "crossr-loops/v0";
    /// The only `kind` this reader accepts.
    pub const KIND: &'static str = "Graph";

    /// The node with `id`, if any.
    #[must_use]
    pub fn node(&self, id: &NodeId) -> Option<&Node> {
        self.nodes.iter().find(|n| &n.id == id)
    }

    /// Out-edges of `id`, in document order.
    pub fn out_edges<'a>(&'a self, id: &'a NodeId) -> impl Iterator<Item = &'a Edge> + 'a {
        self.edges.iter().filter(move |e| &e.from == id)
    }

    /// Sinks: nodes with no out-edges, in document order. `role: terminal`
    /// is a sink by construction; a `gate` with no out-edge is one too.
    #[must_use]
    pub fn sinks(&self) -> Vec<&NodeId> {
        self.nodes
            .iter()
            .map(|n| &n.id)
            .filter(|id| self.out_edges(id).next().is_none())
            .collect()
    }
}

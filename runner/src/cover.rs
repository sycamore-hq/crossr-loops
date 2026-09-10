//! Calculations: edge coverage of a set of walks (decision 6).
//!
//! Replay every walk with descent on and record each `(graph, edge index)` a
//! step traversed, in whichever graph the edge lives — a `flagship` walk
//! covers `avril`, `axel` and `code-gan` edges. What no walk took is
//! uncovered. Edges are counted by `(graph, index)`, so two edges that read
//! alike in different graphs are distinct. Pure: the walks and the graphs
//! come from the caller.

use std::collections::BTreeSet;
use std::fmt;

use thiserror::Error;

use crate::event::Event;
use crate::graph::{Label, NodeId};
use crate::load::Graphs;
use crate::step::{self, WalkError};

/// One edge no walk took.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Uncovered {
    /// The graph the edge lives in.
    pub graph: String,
    /// Source node.
    pub from: NodeId,
    /// Edge label; `None` for an unlabeled edge (fires on `next`).
    pub when: Option<Label>,
    /// Target node.
    pub to: NodeId,
}

impl fmt::Display for Uncovered {
    /// `<graph>: <from> --<when>--> <to>`, `next` for an unlabeled edge —
    /// the same line a trace would print had a walk taken it.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let when = self.when.as_ref().map_or(Label::NEXT, Label::as_str);
        write!(f, "{}: {} --{when}--> {}", self.graph, self.from, self.to)
    }
}

/// What the walks covered.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Coverage {
    /// Every `(graph name, edge index)` some walk traversed.
    pub taken: BTreeSet<(String, usize)>,
    /// Every edge no walk traversed, in graph-name then edge order.
    pub uncovered: Vec<Uncovered>,
    /// Number of edges across every graph in the set.
    pub total: usize,
}

impl Coverage {
    /// `true` when every edge of every graph was taken.
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.uncovered.is_empty()
    }
}

/// Why coverage could not be computed. A walk that does not complete is an
/// error, not partial coverage. `walk` is the position in the caller's list.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum CoverError {
    /// A walk names a graph the set does not hold.
    #[error("walk {walk} names graph '{graph}', which is not loaded")]
    UnknownGraph {
        /// Position in the caller's list.
        walk: usize,
        /// The graph the walk named.
        graph: String,
    },
    /// A walk did not complete; see [`WalkError`].
    #[error("walk {walk} ({graph}): {source}")]
    Walk {
        /// Position in the caller's list.
        walk: usize,
        /// The graph the walk named.
        graph: String,
        /// Why it stopped, with the trace so far (boxed: it carries a trace).
        #[source]
        source: Box<WalkError>,
    },
}

/// Replay every `(graph, events)` in `walks` against `graphs` with descent
/// on and report which edges were taken and which were not.
///
/// # Errors
///
/// [`CoverError::UnknownGraph`] when a walk names a graph not in the set,
/// [`CoverError::Walk`] when a walk does not complete. Either names the
/// walk's position in `walks`.
pub fn cover(graphs: &Graphs, walks: &[(String, Vec<Event>)]) -> Result<Coverage, CoverError> {
    let mut taken: BTreeSet<(String, usize)> = BTreeSet::new();
    for (i, (name, events)) in walks.iter().enumerate() {
        let g = graphs.get(name).ok_or_else(|| CoverError::UnknownGraph {
            walk: i,
            graph: name.clone(),
        })?;
        let trace = step::walk(g, graphs, events, false).map_err(|source| CoverError::Walk {
            walk: i,
            graph: name.clone(),
            source: Box::new(source),
        })?;
        taken.extend(trace.edges().map(|(g, i)| (g.to_owned(), i)));
    }
    let mut uncovered = Vec::new();
    let mut total = 0;
    for g in graphs.values() {
        total += g.edges.len();
        for (index, e) in g.edges.iter().enumerate() {
            if !taken.contains(&(g.name.clone(), index)) {
                uncovered.push(Uncovered {
                    graph: g.name.clone(),
                    from: e.from.clone(),
                    when: e.when.clone(),
                    to: e.to.clone(),
                });
            }
        }
    }
    Ok(Coverage {
        taken,
        uncovered,
        total,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Verdict;
    use crate::load;

    const BLESS: Event = Event::Verdict(Verdict::Bless);
    const REJECT: Event = Event::Verdict(Verdict::Reject);

    /// `outer` descends into `inner` at `sub`; both have a REJECT loop.
    fn graphs() -> Graphs {
        let outer = r#"{
            "apiVersion": "crossr-loops/v0", "kind": "Graph", "name": "outer", "start": "in",
            "nodes": [
                {"id": "in", "role": "gate"},
                {"id": "sub", "role": "graph", "uses": {"graph": "inner"}},
                {"id": "out", "role": "terminal"}
            ],
            "edges": [
                {"from": "in", "to": "sub"},
                {"from": "sub", "to": "out", "when": "done"},
                {"from": "sub", "to": "in", "when": "again"}
            ]
        }"#;
        let inner = r#"{
            "apiVersion": "crossr-loops/v0", "kind": "Graph", "name": "inner", "start": "a",
            "nodes": [
                {"id": "a", "role": "generator"},
                {"id": "b", "role": "adversary"},
                {"id": "stop", "role": "terminal"}
            ],
            "edges": [
                {"from": "a", "to": "b"},
                {"from": "b", "to": "stop", "when": "BLESS"},
                {"from": "b", "to": "a", "when": "REJECT"}
            ]
        }"#;
        let mut graphs = Graphs::new();
        graphs.insert(
            "outer".to_owned(),
            load::parse("outer", outer).expect("outer parses"),
        );
        graphs.insert(
            "inner".to_owned(),
            load::parse("inner", inner).expect("inner parses"),
        );
        graphs
    }

    fn walk(graph: &str, events: &[Event]) -> (String, Vec<Event>) {
        (graph.to_owned(), events.to_vec())
    }

    #[test]
    fn no_walks_leaves_every_edge_uncovered_in_graph_then_edge_order() {
        let c = cover(&graphs(), &[]).expect("no walks is not an error");
        assert!(c.taken.is_empty());
        assert_eq!(c.total, 6);
        assert!(!c.is_full());
        let lines: Vec<String> = c.uncovered.iter().map(ToString::to_string).collect();
        assert_eq!(
            lines,
            [
                "inner: a --next--> b",
                "inner: b --BLESS--> stop",
                "inner: b --REJECT--> a",
                "outer: in --next--> sub",
                "outer: sub --done--> out",
                "outer: sub --again--> in",
            ]
        );
    }

    #[test]
    fn a_descending_walk_covers_edges_in_both_graphs() {
        let happy = walk(
            "outer",
            &[
                Event::Next,
                Event::Next,
                BLESS,
                Event::Label(Label::new("done")),
            ],
        );
        let c = cover(&graphs(), &[happy]).expect("happy path completes");
        assert_eq!(
            c.taken,
            BTreeSet::from([
                ("outer".to_owned(), 0),
                ("inner".to_owned(), 0),
                ("inner".to_owned(), 1),
                ("outer".to_owned(), 1),
            ])
        );
        assert_eq!(
            c.uncovered,
            [
                Uncovered {
                    graph: "inner".to_owned(),
                    from: NodeId::new("b"),
                    when: Some(Label::new("REJECT")),
                    to: NodeId::new("a"),
                },
                Uncovered {
                    graph: "outer".to_owned(),
                    from: NodeId::new("sub"),
                    when: Some(Label::new("again")),
                    to: NodeId::new("in"),
                },
            ]
        );
    }

    #[test]
    fn edges_are_counted_by_graph_and_index() {
        // `inner` alone takes the inner REJECT loop; `outer` alone takes the
        // outer one. Neither is credited to the other graph.
        let inner = walk("inner", &[Event::Next, REJECT, Event::Next, BLESS]);
        let outer = walk(
            "outer",
            &[
                Event::Next,
                Event::Next,
                BLESS,
                Event::Label(Label::new("again")),
                Event::Next,
                Event::Next,
                BLESS,
                Event::Label(Label::new("done")),
            ],
        );
        let c = cover(&graphs(), &[inner, outer]).expect("both complete");
        assert!(c.is_full(), "{:?}", c.uncovered);
        assert_eq!(c.taken.len(), c.total);
    }

    #[test]
    fn an_incomplete_walk_is_an_error_not_partial_coverage() {
        let short = walk("outer", &[Event::Next, Event::Next]);
        let err = cover(&graphs(), &[walk("inner", &[Event::Next, BLESS]), short])
            .expect_err("the second walk stops inside inner");
        match &err {
            CoverError::Walk {
                walk: 1,
                graph,
                source,
            } => {
                assert_eq!(graph, "outer");
                match source.as_ref() {
                    WalkError::Incomplete {
                        graph,
                        depth: 1,
                        at,
                        ..
                    } => {
                        assert_eq!(graph, "inner");
                        assert_eq!(at, &NodeId::new("b"));
                    }
                    other => panic!("expected Incomplete at inner:b, got {other:?}"),
                }
            }
            other => panic!("expected Walk, got {other:?}"),
        }
        assert_eq!(err.to_string(), "walk 1 (outer): incomplete at inner:b");
    }

    #[test]
    fn a_walk_naming_an_unknown_graph_is_an_error() {
        let err = cover(&graphs(), &[walk("nowhere", &[])]).expect_err("no such graph");
        assert_eq!(
            err,
            CoverError::UnknownGraph {
                walk: 0,
                graph: "nowhere".to_owned(),
            }
        );
        assert_eq!(
            err.to_string(),
            "walk 0 names graph 'nowhere', which is not loaded"
        );
    }
}

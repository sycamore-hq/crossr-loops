//! Data: the path a walk took.
//!
//! One [`Step`] per event consumed, plus one per descent into a subgraph and
//! one per return from it. [`Trace`] renders one line per step, indented two
//! spaces per level of depth, in the graphs' own vocabulary — nothing a
//! conductor could read as a verdict or an instruction:
//!
//! ```text
//! <graph>: <from> --<event>--> <to>
//! <graph>: <node> >> <sub>
//! <sub>: <sink> << <graph>:<node>
//! ```

use std::fmt;

use crate::event::Event;
use crate::graph::NodeId;

/// One move of a walk.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Step {
    /// An edge traversal: one event consumed.
    Edge {
        /// Nesting level; 0 in the graph the walk started in.
        depth: usize,
        /// The graph the edge lives in.
        graph: String,
        /// Position of the edge in that graph's `edges`.
        index: usize,
        /// Source node.
        from: NodeId,
        /// The event that selected the edge.
        event: Event,
        /// Target node.
        to: NodeId,
    },
    /// Arrival at a `role: graph` node opened its subgraph at that graph's
    /// `start`. No event consumed.
    Descend {
        /// Depth of the parent graph.
        depth: usize,
        /// The parent graph.
        graph: String,
        /// The `role: graph` node.
        node: NodeId,
        /// The subgraph entered.
        sub: String,
    },
    /// A subgraph stood on a sink and the next event belonged to the parent,
    /// so the frame closed. No event consumed.
    Return {
        /// Depth of the subgraph being left.
        depth: usize,
        /// The subgraph being left.
        sub: String,
        /// The sink it stood on.
        sink: NodeId,
        /// The parent graph.
        graph: String,
        /// The `role: graph` node the walk stands on again.
        node: NodeId,
    },
}

impl Step {
    /// Nesting level of the line: 0 in the graph the walk started in.
    #[must_use]
    pub fn depth(&self) -> usize {
        match self {
            Step::Edge { depth, .. } | Step::Descend { depth, .. } | Step::Return { depth, .. } => {
                *depth
            }
        }
    }

    /// `(graph, edge index)` when this step traversed an edge.
    #[must_use]
    pub fn edge(&self) -> Option<(&str, usize)> {
        match self {
            Step::Edge { graph, index, .. } => Some((graph, *index)),
            Step::Descend { .. } | Step::Return { .. } => None,
        }
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..self.depth() {
            f.write_str("  ")?;
        }
        match self {
            Step::Edge {
                graph,
                from,
                event,
                to,
                ..
            } => write!(f, "{graph}: {from} --{event}--> {to}"),
            Step::Descend {
                graph, node, sub, ..
            } => write!(f, "{graph}: {node} >> {sub}"),
            Step::Return {
                sub,
                sink,
                graph,
                node,
                ..
            } => write!(f, "{sub}: {sink} << {graph}:{node}"),
        }
    }
}

/// Where a trace ends.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum End {
    /// On a sink at depth 0 with every event consumed (decision 4).
    Complete {
        /// The sink.
        at: NodeId,
    },
    /// The walk stopped here without completing; the accompanying
    /// [`crate::step::WalkError`] says why and names the frame.
    Stopped {
        /// The node the walk stood on when it stopped (innermost frame).
        at: NodeId,
    },
}

impl End {
    /// The node the trace ends on.
    #[must_use]
    pub fn at(&self) -> &NodeId {
        match self {
            End::Complete { at } | End::Stopped { at } => at,
        }
    }
}

/// The steps a walk took, in order, and where it ended.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trace {
    /// Moves, in order. Empty when no event was consumed and nothing opened.
    pub steps: Vec<Step>,
    /// Where the walk stands.
    pub end: End,
}

impl Trace {
    /// The node the trace ends on.
    #[must_use]
    pub fn at(&self) -> &NodeId {
        self.end.at()
    }

    /// `true` when the walk reached a sink at depth 0 with every event
    /// consumed.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        matches!(self.end, End::Complete { .. })
    }

    /// The edges traversed, as `(graph, edge index)`, in order.
    pub fn edges(&self) -> impl Iterator<Item = (&str, usize)> + '_ {
        self.steps.iter().filter_map(Step::edge)
    }
}

impl fmt::Display for Trace {
    /// One line per step, each newline-terminated; nothing for the end, so
    /// the last line of a complete walk names its sink.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for step in &self.steps {
            writeln!(f, "{step}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Verdict;
    use crate::graph::Label;

    fn edge(depth: usize, graph: &str, index: usize, from: &str, event: Event, to: &str) -> Step {
        Step::Edge {
            depth,
            graph: graph.to_owned(),
            index,
            from: NodeId::new(from),
            event,
            to: NodeId::new(to),
        }
    }

    #[test]
    fn renders_one_line_per_edge_in_the_graphs_vocabulary() {
        let trace = Trace {
            steps: vec![
                edge(0, "g", 0, "a", Event::Next, "b"),
                edge(0, "g", 1, "b", Event::Label(Label::new("pass")), "c"),
                edge(0, "g", 2, "c", Event::Verdict(Verdict::Bless), "stop"),
            ],
            end: End::Complete {
                at: NodeId::new("stop"),
            },
        };
        assert_eq!(
            trace.to_string(),
            "g: a --next--> b\ng: b --pass--> c\ng: c --BLESS--> stop\n"
        );
        assert!(trace.is_complete());
        assert_eq!(trace.at(), &NodeId::new("stop"));
        assert_eq!(
            trace.edges().collect::<Vec<_>>(),
            [("g", 0), ("g", 1), ("g", 2)]
        );
    }

    #[test]
    fn renders_descent_and_return_indented_two_spaces_per_level() {
        let trace = Trace {
            steps: vec![
                edge(0, "outer", 0, "in", Event::Next, "sub"),
                Step::Descend {
                    depth: 0,
                    graph: "outer".to_owned(),
                    node: NodeId::new("sub"),
                    sub: "inner".to_owned(),
                },
                edge(1, "inner", 0, "a", Event::Next, "b"),
                Step::Descend {
                    depth: 1,
                    graph: "inner".to_owned(),
                    node: NodeId::new("b"),
                    sub: "leaf".to_owned(),
                },
                edge(2, "leaf", 0, "x", Event::Label(Label::new("go")), "y"),
                Step::Return {
                    depth: 2,
                    sub: "leaf".to_owned(),
                    sink: NodeId::new("y"),
                    graph: "inner".to_owned(),
                    node: NodeId::new("b"),
                },
                edge(1, "inner", 1, "b", Event::Verdict(Verdict::Bless), "stop"),
                Step::Return {
                    depth: 1,
                    sub: "inner".to_owned(),
                    sink: NodeId::new("stop"),
                    graph: "outer".to_owned(),
                    node: NodeId::new("sub"),
                },
                edge(
                    0,
                    "outer",
                    1,
                    "sub",
                    Event::Label(Label::new("done")),
                    "out",
                ),
            ],
            end: End::Complete {
                at: NodeId::new("out"),
            },
        };
        assert_eq!(
            trace.to_string(),
            "outer: in --next--> sub\n\
             outer: sub >> inner\n\
             \x20 inner: a --next--> b\n\
             \x20 inner: b >> leaf\n\
             \x20   leaf: x --go--> y\n\
             \x20   leaf: y << inner:b\n\
             \x20 inner: b --BLESS--> stop\n\
             \x20 inner: stop << outer:sub\n\
             outer: sub --done--> out\n"
        );
        assert_eq!(trace.steps[1].depth(), 0);
        assert_eq!(trace.steps[5].depth(), 2);
        assert_eq!(trace.steps[1].edge(), None);
        assert_eq!(
            trace.edges().collect::<Vec<_>>(),
            [
                ("outer", 0),
                ("inner", 0),
                ("leaf", 0),
                ("inner", 1),
                ("outer", 1)
            ]
        );
    }

    #[test]
    fn empty_trace_renders_nothing_and_ends_where_it_stands() {
        let trace = Trace {
            steps: Vec::new(),
            end: End::Stopped {
                at: NodeId::new("a"),
            },
        };
        assert_eq!(trace.to_string(), "");
        assert!(!trace.is_complete());
        assert_eq!(trace.at(), &NodeId::new("a"));
        assert_eq!(trace.edges().count(), 0);
    }
}

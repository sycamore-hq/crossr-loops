//! Data: the path a walk took.
//!
//! One [`Step`] per event consumed. [`Trace`] renders as
//! `<graph>: <from> --<event>--> <to>`, one line per step — the graphs' own
//! vocabulary, nothing a conductor could read as a verdict or an instruction.

use std::fmt;

use crate::event::Event;
use crate::graph::NodeId;

/// One edge traversal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Step {
    /// The graph the edge lives in.
    pub graph: String,
    /// Source node.
    pub from: NodeId,
    /// The event that selected the edge.
    pub event: Event,
    /// Target node.
    pub to: NodeId,
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} --{}--> {}",
            self.graph, self.from, self.event, self.to
        )
    }
}

/// Where a trace ends.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum End {
    /// On a sink with every event consumed (decision 4).
    Complete {
        /// The sink.
        at: NodeId,
    },
    /// The walk stopped here without completing; the accompanying
    /// [`crate::step::WalkError`] says why.
    Stopped {
        /// The node the walk stood on when it stopped.
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
    /// Edge traversals, in order. Empty when no event was consumed.
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

    /// `true` when the walk reached a sink with every event consumed.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        matches!(self.end, End::Complete { .. })
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

    fn step(from: &str, event: Event, to: &str) -> Step {
        Step {
            graph: "g".to_owned(),
            from: NodeId::new(from),
            event,
            to: NodeId::new(to),
        }
    }

    #[test]
    fn renders_one_line_per_edge_in_the_graphs_vocabulary() {
        let trace = Trace {
            steps: vec![
                step("a", Event::Next, "b"),
                step("b", Event::Label(Label::new("pass")), "c"),
                step("c", Event::Verdict(Verdict::Bless), "stop"),
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
    }
}

//! Calculations: stepping semantics (decisions 3 and 4).
//!
//! Pure over a loaded [`Graph`]: no I/O, no clock, no model. [`step`] moves
//! one node on one event; [`walk`] replays a whole event list from the
//! graph's `start` and says whether it reached a sink.
//!
//! Ambiguous out-edges are not modelled here: [`crate::load::check`] refuses
//! a graph with two out-edges of one node that the same event could fire, so
//! the first match is the only match.

use thiserror::Error;

use crate::event::{Event, Verdict};
use crate::graph::{Graph, Label, NodeId, Role};
use crate::trace::{End, Step, Trace};

/// Why one step could not be taken. Every variant names the graph and node.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum StepError {
    /// No out-edge of `node` fires on `event`.
    #[error("{graph}: {node} accepts {}; got {event}", describe_accepted(accepted))]
    NoEdge {
        /// Graph name.
        graph: String,
        /// The node the walk stood on.
        node: NodeId,
        /// The event that matched nothing.
        event: Event,
        /// The node's labels, in edge order; [`Label::NEXT`] for an
        /// unlabeled edge. Empty on a sink.
        accepted: Vec<String>,
    },
    /// An `adversary` node was given something other than `BLESS` /
    /// `REJECT`. Fires even when a label would have matched: an adversary
    /// accepts a verdict and nothing else.
    #[error(
        "{graph}: {node} is an adversary and accepts {}; got {event}",
        Verdict::accepted()
    )]
    NotAVerdict {
        /// Graph name.
        graph: String,
        /// The adversary node.
        node: NodeId,
        /// The non-verdict event.
        event: Event,
    },
}

fn describe_accepted(accepted: &[String]) -> String {
    if accepted.is_empty() {
        "nothing (sink)".to_owned()
    } else {
        accepted.join(", ")
    }
}

/// Why a walk did not complete. Every variant carries the trace so far
/// (boxed: the error must stay small next to the `Ok` it displaces).
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum WalkError {
    /// A step failed; see [`StepError`].
    #[error("{source}")]
    Step {
        /// Steps taken before the failure.
        trace: Box<Trace>,
        /// The failing step.
        #[source]
        source: StepError,
    },
    /// A sink was reached with events left over (decision 4).
    #[error(
        "trailing events after sink {graph}:{at}: {}",
        describe_events(remaining)
    )]
    TrailingEvents {
        /// Graph name.
        graph: String,
        /// Steps taken to the sink.
        trace: Box<Trace>,
        /// The sink.
        at: NodeId,
        /// Events not consumed, in order.
        remaining: Vec<Event>,
    },
    /// Events ran out before a sink (decision 4).
    #[error("incomplete at {graph}:{at}")]
    Incomplete {
        /// Graph name.
        graph: String,
        /// Steps taken.
        trace: Box<Trace>,
        /// The non-sink node the walk stood on.
        at: NodeId,
    },
}

impl WalkError {
    /// The steps taken before the walk stopped.
    #[must_use]
    pub fn trace(&self) -> &Trace {
        match self {
            WalkError::Step { trace, .. }
            | WalkError::TrailingEvents { trace, .. }
            | WalkError::Incomplete { trace, .. } => trace,
        }
    }
}

fn describe_events(events: &[Event]) -> String {
    events
        .iter()
        .map(Event::as_str)
        .collect::<Vec<_>>()
        .join(" ")
}

/// `true` when `node` has no out-edges (decision 3). `role: terminal` is a
/// sink by construction; a `gate` with no out-edge is one too.
#[must_use]
pub fn is_sink(g: &Graph, node: &NodeId) -> bool {
    g.out_edges(node).next().is_none()
}

/// The events `node` accepts, in edge order: each out-edge's `when`, or
/// [`Label::NEXT`] for an unlabeled edge. Empty on a sink.
#[must_use]
pub fn accepted(g: &Graph, node: &NodeId) -> Vec<String> {
    g.out_edges(node)
        .map(|e| {
            e.when
                .as_ref()
                .map_or(Label::NEXT, Label::as_str)
                .to_owned()
        })
        .collect()
}

/// Take one step from `node` on `event`.
///
/// `Next` fires an unlabeled edge, `BLESS` / `REJECT` fire the edge with
/// that label, any other label fires the edge whose `when` equals it. An
/// `adversary` node accepts a verdict and nothing else.
///
/// # Errors
///
/// [`StepError::NotAVerdict`] when `node` is an adversary and `event` is not
/// `BLESS` / `REJECT`; [`StepError::NoEdge`] when no out-edge fires, naming
/// what the node accepts.
pub fn step(g: &Graph, node: &NodeId, event: &Event) -> Result<NodeId, StepError> {
    let is_adversary = g
        .node(node)
        .is_some_and(|n| n.role == Some(Role::Adversary));
    if is_adversary && !event.is_verdict() {
        return Err(StepError::NotAVerdict {
            graph: g.name.clone(),
            node: node.clone(),
            event: event.clone(),
        });
    }
    g.out_edges(node)
        .find(|e| event.matches(e.when.as_ref()))
        .map(|e| e.to.clone())
        .ok_or_else(|| StepError::NoEdge {
            graph: g.name.clone(),
            node: node.clone(),
            event: event.clone(),
            accepted: accepted(g, node),
        })
}

/// Replay `events` from the graph's `start` (never `nodes[0]`).
///
/// Complete iff the walk stands on a sink with every event consumed
/// (decision 4).
///
/// # Errors
///
/// [`WalkError::TrailingEvents`] when a sink is reached with events left,
/// [`WalkError::Incomplete`] when events run out before a sink,
/// [`WalkError::Step`] when a step fails. Each carries the trace so far.
pub fn walk(g: &Graph, events: &[Event]) -> Result<Trace, WalkError> {
    let mut at = g.start.clone();
    let mut steps: Vec<Step> = Vec::with_capacity(events.len());
    let stopped = |steps: Vec<Step>, at: &NodeId| {
        Box::new(Trace {
            steps,
            end: End::Stopped { at: at.clone() },
        })
    };
    for (i, event) in events.iter().enumerate() {
        if is_sink(g, &at) {
            return Err(WalkError::TrailingEvents {
                graph: g.name.clone(),
                trace: stopped(steps, &at),
                at,
                remaining: events[i..].to_vec(),
            });
        }
        match step(g, &at, event) {
            Ok(to) => {
                steps.push(Step {
                    graph: g.name.clone(),
                    from: at,
                    event: event.clone(),
                    to: to.clone(),
                });
                at = to;
            }
            Err(source) => {
                return Err(WalkError::Step {
                    trace: stopped(steps, &at),
                    source,
                })
            }
        }
    }
    if is_sink(g, &at) {
        Ok(Trace {
            steps,
            end: End::Complete { at },
        })
    } else {
        Err(WalkError::Incomplete {
            graph: g.name.clone(),
            trace: stopped(steps, &at),
            at,
        })
    }
}

//! Calculations: stepping semantics (decisions 3, 4 and 5).
//!
//! Pure over loaded [`Graph`] values: no I/O, no clock, no model. [`step`]
//! moves one node on one event; [`walk`] replays a whole event list from the
//! graph's `start` and says whether it reached a sink.
//!
//! Descent (decision 5): arriving at a `role: graph` node opens the named
//! subgraph at its `start`. Subgraphs come from a [`Resolve`] the caller
//! supplies, so the walk itself never touches a file. When a subgraph stands
//! on a sink, the next event belongs to the parent: the frame closes and the
//! event selects the parent node's out-edge. `flat` turns descent off and
//! treats a `graph` node as any other node.
//!
//! Ambiguous out-edges are not modelled here: [`crate::load::check`] refuses
//! a graph with two out-edges of one node that the same event could fire, so
//! the first match is the only match.

use thiserror::Error;

use crate::event::{Event, Verdict};
use crate::graph::{Edge, Graph, Label, NodeId, Role};
use crate::trace::{End, Step, Trace};

/// Graph name → graph. [`walk`] asks this for every `uses.graph` it
/// descends into; [`crate::load::Graphs`] is the usual implementation.
pub trait Resolve {
    /// The graph named `name`, if this resolver holds it.
    fn resolve(&self, name: &str) -> Option<&Graph>;
}

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
    /// A `role: graph` node names a subgraph the [`Resolve`] does not hold.
    /// [`crate::load::load_dir`] refuses such a directory up front; this
    /// fires only for a resolver assembled some other way.
    #[error("{graph}: {node} descends into '{sub}', which is not loaded")]
    UnresolvedSubgraph {
        /// Graph name.
        graph: String,
        /// The `role: graph` node.
        node: NodeId,
        /// The `uses.graph` value.
        sub: String,
    },
    /// A `role: graph` node names a graph that is already open on the walk's
    /// stack, so descending would never consume an event.
    #[error("{graph}: {node} descends into '{sub}', which is already open")]
    RecursiveDescent {
        /// Graph name.
        graph: String,
        /// The `role: graph` node.
        node: NodeId,
        /// The `uses.graph` value.
        sub: String,
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
    /// A sink was reached at depth 0 with events left over (decision 4).
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
    /// Events ran out before a sink at depth 0 (decision 4). Names the
    /// innermost frame: a walk that ends on a subgraph's sink is incomplete
    /// there, at that depth.
    #[error("incomplete at {graph}:{at}")]
    Incomplete {
        /// Graph of the innermost frame.
        graph: String,
        /// Depth of the innermost frame; 0 in the graph the walk started in.
        depth: usize,
        /// Steps taken.
        trace: Box<Trace>,
        /// The node the walk stood on.
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

/// The subgraph `node` descends into: its `uses.graph` when the node is
/// `role: graph`. `None` for every other node, and for a `graph` node that
/// names no subgraph (opaque).
#[must_use]
pub fn subgraph_of<'g>(g: &'g Graph, node: &NodeId) -> Option<&'g str> {
    let n = g.node(node)?;
    if n.role != Some(Role::Graph) {
        return None;
    }
    n.uses.as_ref()?.graph.as_deref()
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

/// The out-edge of `node` that `event` fires, with its position in
/// `edges`. The rules are those of [`step`].
///
/// # Errors
///
/// As [`step`].
pub fn select_edge<'g>(
    g: &'g Graph,
    node: &NodeId,
    event: &Event,
) -> Result<(usize, &'g Edge), StepError> {
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
    g.edges
        .iter()
        .enumerate()
        .find(|(_, e)| &e.from == node && event.matches(e.when.as_ref()))
        .ok_or_else(|| StepError::NoEdge {
            graph: g.name.clone(),
            node: node.clone(),
            event: event.clone(),
            accepted: accepted(g, node),
        })
}

/// Take one step from `node` on `event`.
///
/// `Next` fires an unlabeled edge, `BLESS` / `REJECT` fire the edge with
/// that label, any other label fires the edge whose `when` equals it. An
/// `adversary` node accepts a verdict and nothing else. Descent is not a
/// step: a `role: graph` node steps like any other node here.
///
/// # Errors
///
/// [`StepError::NotAVerdict`] when `node` is an adversary and `event` is not
/// `BLESS` / `REJECT`; [`StepError::NoEdge`] when no out-edge fires, naming
/// what the node accepts.
pub fn step(g: &Graph, node: &NodeId, event: &Event) -> Result<NodeId, StepError> {
    select_edge(g, node, event).map(|(_, e)| e.to.clone())
}

/// One open graph on the walk's stack. Depth is the frame's index.
struct Frame<'g> {
    graph: &'g Graph,
    at: NodeId,
}

/// The trace so far, stopped on the innermost frame. The bottom frame never
/// closes, so `frames` is never empty.
fn stopped(steps: Vec<Step>, frames: &[Frame<'_>]) -> Trace {
    let at = frames[frames.len() - 1].at.clone();
    Trace {
        steps,
        end: End::Stopped { at },
    }
}

/// While the innermost frame stands on a `role: graph` node, open its
/// subgraph at that graph's `start` (decision 5). Records one
/// [`Step::Descend`] per level. Does nothing when `flat`.
fn descend<'g>(
    frames: &mut Vec<Frame<'g>>,
    steps: &mut Vec<Step>,
    resolve: &'g dyn Resolve,
    flat: bool,
) -> Result<(), StepError> {
    if flat {
        return Ok(());
    }
    while let Some(top) = frames.last() {
        let Some(sub) = subgraph_of(top.graph, &top.at) else {
            return Ok(());
        };
        let graph = top.graph.name.clone();
        let node = top.at.clone();
        let Some(sub_graph) = resolve.resolve(sub) else {
            return Err(StepError::UnresolvedSubgraph {
                graph,
                node,
                sub: sub.to_owned(),
            });
        };
        if frames.iter().any(|f| std::ptr::eq(f.graph, sub_graph)) {
            return Err(StepError::RecursiveDescent {
                graph,
                node,
                sub: sub.to_owned(),
            });
        }
        steps.push(Step::Descend {
            depth: frames.len() - 1,
            graph,
            node,
            sub: sub.to_owned(),
        });
        frames.push(Frame {
            graph: sub_graph,
            at: sub_graph.start.clone(),
        });
    }
    Ok(())
}

/// While the innermost frame is a subgraph standing on a sink, close it: the
/// walk stands on the parent's `role: graph` node again and the coming event
/// selects that node's out-edge (decision 5). Records one [`Step::Return`]
/// per level. The bottom frame never closes.
fn unwind(frames: &mut Vec<Frame<'_>>, steps: &mut Vec<Step>) {
    while frames.len() > 1 {
        let top = &frames[frames.len() - 1];
        if !is_sink(top.graph, &top.at) {
            return;
        }
        let sub = top.graph.name.clone();
        let sink = top.at.clone();
        frames.pop();
        let parent = &frames[frames.len() - 1];
        steps.push(Step::Return {
            depth: frames.len(),
            sub,
            sink,
            graph: parent.graph.name.clone(),
            node: parent.at.clone(),
        });
    }
}

/// Replay `events` from the graph's `start` (never `nodes[0]`), descending
/// into subgraphs through `resolve` unless `flat`.
///
/// Complete iff the walk stands on a sink at depth 0 with every event
/// consumed (decision 4).
///
/// # Errors
///
/// [`WalkError::TrailingEvents`] when a sink at depth 0 is reached with
/// events left, [`WalkError::Incomplete`] when events run out before one
/// (naming the innermost frame and its depth), [`WalkError::Step`] when a
/// step or a descent fails. Each carries the trace so far.
pub fn walk<'g>(
    g: &'g Graph,
    resolve: &'g dyn Resolve,
    events: &[Event],
    flat: bool,
) -> Result<Trace, WalkError> {
    let mut frames: Vec<Frame<'g>> = vec![Frame {
        graph: g,
        at: g.start.clone(),
    }];
    let mut steps: Vec<Step> = Vec::with_capacity(events.len());
    if let Err(source) = descend(&mut frames, &mut steps, resolve, flat) {
        return Err(WalkError::Step {
            trace: Box::new(stopped(steps, &frames)),
            source,
        });
    }
    for (i, event) in events.iter().enumerate() {
        unwind(&mut frames, &mut steps);
        let depth = frames.len() - 1;
        let top = &frames[depth];
        if is_sink(top.graph, &top.at) {
            let at = top.at.clone();
            return Err(WalkError::TrailingEvents {
                graph: top.graph.name.clone(),
                trace: Box::new(stopped(steps, &frames)),
                at,
                remaining: events[i..].to_vec(),
            });
        }
        let (index, edge) = match select_edge(top.graph, &top.at, event) {
            Ok(found) => found,
            Err(source) => {
                return Err(WalkError::Step {
                    trace: Box::new(stopped(steps, &frames)),
                    source,
                })
            }
        };
        steps.push(Step::Edge {
            depth,
            graph: top.graph.name.clone(),
            index,
            from: top.at.clone(),
            event: event.clone(),
            to: edge.to.clone(),
        });
        frames[depth].at = edge.to.clone();
        if let Err(source) = descend(&mut frames, &mut steps, resolve, flat) {
            return Err(WalkError::Step {
                trace: Box::new(stopped(steps, &frames)),
                source,
            });
        }
    }
    let depth = frames.len() - 1;
    let top = &frames[depth];
    if depth == 0 && is_sink(top.graph, &top.at) {
        Ok(Trace {
            steps,
            end: End::Complete { at: top.at.clone() },
        })
    } else {
        let at = top.at.clone();
        Err(WalkError::Incomplete {
            graph: top.graph.name.clone(),
            depth,
            trace: Box::new(stopped(steps, &frames)),
            at,
        })
    }
}

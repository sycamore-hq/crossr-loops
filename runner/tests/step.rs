//! Stepping tests (decisions 3 and 4): the six C-08 cases against fixture
//! graphs, plus the grammar of what `step` / `walk` report, checked against
//! fixtures and the committed graphs.

use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use graph_runner::event::{Event, Verdict};
use graph_runner::graph::{Graph, Label, NodeId, Role};
use graph_runner::load::{self, Graphs};
use graph_runner::step::{self, StepError, WalkError};
use graph_runner::trace::{End, Step, Trace};

fn graphs_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("graphs")
        .canonicalize()
        .expect("runner/ lives one level below the repo root")
}

fn committed(name: &str) -> Graph {
    load::load_file(&graphs_dir().join(format!("{name}.json"))).expect("committed graph loads")
}

/// A descent-free walk: no graph here has a `role: graph` node, so an empty
/// resolver is never consulted. Descent itself is `tests/descent.rs`.
fn walk(g: &Graph, events: &[Event]) -> Result<Trace, WalkError> {
    step::walk(g, &Graphs::new(), events, false)
}

fn from_of(step: &Step) -> &NodeId {
    match step {
        Step::Edge { from, .. } => from,
        other => panic!("expected an edge step, got {other:?}"),
    }
}

fn to_of(step: &Step) -> &NodeId {
    match step {
        Step::Edge { to, .. } => to,
        other => panic!("expected an edge step, got {other:?}"),
    }
}

fn id(s: &str) -> NodeId {
    NodeId::new(s)
}

fn label(s: &str) -> Event {
    Event::Label(Label::new(s))
}

const BLESS: Event = Event::Verdict(Verdict::Bless);
const REJECT: Event = Event::Verdict(Verdict::Reject);

/// generator `a` → adversary `b` → terminal `stop`; `REJECT` loops to `a`.
fn fixture() -> Value {
    json!({
        "apiVersion": "crossr-loops/v0",
        "kind": "Graph",
        "name": "fixture",
        "start": "a",
        "nodes": [
            {"id": "a", "role": "generator"},
            {"id": "b", "role": "adversary", "uses": {"persona": "someone"}},
            {"id": "stop", "role": "terminal"}
        ],
        "edges": [
            {"from": "a", "to": "b"},
            {"from": "b", "to": "stop", "when": "BLESS"},
            {"from": "b", "to": "a", "when": "REJECT"}
        ]
    })
}

fn parse(v: &Value) -> Graph {
    load::parse("fixture", &v.to_string()).expect("fixture passes the loader")
}

/// The six tests C-08 names. They live in one module so
/// `cargo test -p graph-runner step_` selects exactly these.
mod step_semantics {
    use super::*;

    #[test]
    fn step_unlabeled_edge_fires_on_next() {
        let g = parse(&fixture());
        assert_eq!(step::step(&g, &id("a"), &Event::Next), Ok(id("b")));
        for wrong in [BLESS, REJECT, label("pass")] {
            let err =
                step::step(&g, &id("a"), &wrong).expect_err("only next fires an unlabeled edge");
            assert_eq!(
                err,
                StepError::NoEdge {
                    graph: "fixture".to_owned(),
                    node: id("a"),
                    event: wrong.clone(),
                    accepted: vec!["next".to_owned()],
                },
                "accepted must list next for an unlabeled edge"
            );
            assert_eq!(
                err.to_string(),
                format!("fixture: a accepts next; got {wrong}")
            );
        }
    }

    #[test]
    fn step_adversary_rejects_non_verdict() {
        // A hand-written graph the loader accepts but verify-protocol would
        // not: the adversary has a `fail` edge. The rule still wins.
        let mut v = fixture();
        v["edges"]
            .as_array_mut()
            .expect("edges is an array")
            .push(json!({"from": "b", "to": "a", "when": "fail"}));
        let g = parse(&v);
        assert_eq!(g.node(&id("b")).and_then(|n| n.role), Some(Role::Adversary));

        let err = step::step(&g, &id("b"), &label("fail"))
            .expect_err("a matching label does not rescue a non-verdict");
        assert_eq!(
            err,
            StepError::NotAVerdict {
                graph: "fixture".to_owned(),
                node: id("b"),
                event: label("fail"),
            }
        );
        assert_eq!(
            err.to_string(),
            "fixture: b is an adversary and accepts BLESS, REJECT; got fail"
        );

        let err = step::step(&g, &id("b"), &Event::Next).expect_err("next is not a verdict");
        assert!(matches!(err, StepError::NotAVerdict { .. }), "{err}");

        assert_eq!(step::step(&g, &id("b"), &BLESS), Ok(id("stop")));
        assert_eq!(step::step(&g, &id("b"), &REJECT), Ok(id("a")));
    }

    #[test]
    fn step_start_is_the_start_key() {
        // `nodes[0]` is a decoy; `start` names the second node.
        let mut v = fixture();
        v["nodes"]
            .as_array_mut()
            .expect("nodes is an array")
            .insert(0, json!({"id": "decoy", "role": "stage"}));
        v["edges"]
            .as_array_mut()
            .expect("edges is an array")
            .push(json!({"from": "decoy", "to": "a"}));
        let g = parse(&v);
        assert_eq!(g.nodes[0].id, id("decoy"));
        assert_eq!(g.start, id("a"));

        let trace = walk(&g, &[Event::Next, BLESS]).expect("walk completes");
        assert_eq!(
            from_of(&trace.steps[0]),
            &id("a"),
            "the walk begins at start, not nodes[0]"
        );
        assert_eq!(
            trace.to_string(),
            "fixture: a --next--> b\nfixture: b --BLESS--> stop\n"
        );

        let err = walk(&g, &[Event::Next, Event::Next])
            .expect_err("decoy edge is unreachable from start");
        assert!(matches!(err, WalkError::Step { .. }), "{err}");
    }

    #[test]
    fn step_sink_has_no_out_edges() {
        let mut v = fixture();
        // A `gate` with no out-edge is a sink too (code-gan's `commit`).
        v["nodes"]
            .as_array_mut()
            .expect("nodes is an array")
            .push(json!({"id": "commit", "role": "gate"}));
        let g = parse(&v);

        assert!(step::is_sink(&g, &id("stop")), "terminal is a sink");
        assert!(
            step::is_sink(&g, &id("commit")),
            "gate without out-edges is a sink"
        );
        assert!(!step::is_sink(&g, &id("a")));
        assert!(!step::is_sink(&g, &id("b")));
        assert_eq!(g.sinks(), [&id("stop"), &id("commit")]);

        let err = step::step(&g, &id("stop"), &Event::Next).expect_err("nothing leaves a sink");
        assert_eq!(
            err,
            StepError::NoEdge {
                graph: "fixture".to_owned(),
                node: id("stop"),
                event: Event::Next,
                accepted: vec![],
            }
        );
        assert_eq!(
            err.to_string(),
            "fixture: stop accepts nothing (sink); got next"
        );
    }

    #[test]
    fn walk_trailing_events_fail() {
        let g = parse(&fixture());
        let err = walk(&g, &[Event::Next, BLESS, Event::Next, REJECT])
            .expect_err("events after the sink are an error");
        match &err {
            WalkError::TrailingEvents {
                graph,
                trace,
                at,
                remaining,
            } => {
                assert_eq!(graph, "fixture");
                assert_eq!(at, &id("stop"));
                assert_eq!(remaining, &[Event::Next, REJECT]);
                assert_eq!(trace.steps.len(), 2, "the trace reaches the sink");
                assert_eq!(trace.end, End::Stopped { at: id("stop") });
                assert!(!trace.is_complete());
            }
            other => panic!("expected TrailingEvents, got {other:?}"),
        }
        assert_eq!(
            err.to_string(),
            "trailing events after sink fixture:stop: next REJECT"
        );
        assert_eq!(
            err.trace().to_string(),
            "fixture: a --next--> b\nfixture: b --BLESS--> stop\n"
        );
    }

    #[test]
    fn walk_incomplete_names_node() {
        let g = parse(&fixture());
        let err = walk(&g, &[Event::Next]).expect_err("b is not a sink");
        match &err {
            WalkError::Incomplete {
                graph, trace, at, ..
            } => {
                assert_eq!(graph, "fixture");
                assert_eq!(at, &id("b"));
                assert_eq!(trace.steps.len(), 1);
                assert_eq!(trace.at(), &id("b"));
            }
            other => panic!("expected Incomplete, got {other:?}"),
        }
        assert_eq!(err.to_string(), "incomplete at fixture:b");

        let err = walk(&g, &[]).expect_err("no events, start is not a sink");
        assert_eq!(err.to_string(), "incomplete at fixture:a");
        assert!(err.trace().steps.is_empty());
    }
}

// ---- the rest of the grammar ---------------------------------------------

#[test]
fn walk_happy_path_completes_on_the_sink() {
    let g = parse(&fixture());
    let trace = walk(&g, &[Event::Next, BLESS]).expect("completes");
    assert!(trace.is_complete());
    assert_eq!(trace.end, End::Complete { at: id("stop") });
    assert_eq!(trace.at(), &id("stop"));

    let trace =
        walk(&g, &[Event::Next, REJECT, Event::Next, BLESS]).expect("a REJECT loop then BLESS");
    assert_eq!(trace.steps.len(), 4);
    assert_eq!(to_of(&trace.steps[1]), &id("a"));
    assert!(trace.is_complete());
}

#[test]
fn walk_error_carries_the_partial_trace() {
    let g = parse(&fixture());
    let err = walk(&g, &[Event::Next, label("fail"), BLESS]).expect_err("fail at the adversary");
    match &err {
        WalkError::Step { trace, source } => {
            assert_eq!(trace.steps.len(), 1);
            assert_eq!(trace.end, End::Stopped { at: id("b") });
            assert!(matches!(source, StepError::NotAVerdict { node, .. } if node == &id("b")));
        }
        other => panic!("expected Step, got {other:?}"),
    }
    assert_eq!(
        err.to_string(),
        "fixture: b is an adversary and accepts BLESS, REJECT; got fail"
    );
    assert_eq!(err.trace().to_string(), "fixture: a --next--> b\n");
}

#[test]
fn no_edge_lists_accepted_labels_in_edge_order() {
    // code-gan `generate` has one unlabeled edge and one labeled edge.
    let g = committed("code-gan");
    assert_eq!(
        step::accepted(&g, &id("generate")),
        ["next", "unsatisfiable-claim"]
    );
    let err = step::step(&g, &id("generate"), &label("bogus")).expect_err("no such edge");
    assert_eq!(
        err.to_string(),
        "code-gan: generate accepts next, unsatisfiable-claim; got bogus"
    );
    assert_eq!(step::accepted(&g, &id("mechanical")), ["pass", "fail"]);
    assert_eq!(step::accepted(&g, &id("tester")), ["BLESS", "REJECT"]);
    assert!(step::accepted(&g, &id("commit")).is_empty());
}

#[test]
fn committed_avril_trace_follows_the_line_grammar() {
    let g = committed("avril");
    let trace = walk(&g, &[Event::Next, BLESS, BLESS, BLESS]).expect("avril happy path");
    assert_eq!(
        trace.to_string(),
        "avril: generator --next--> po\n\
         avril: po --BLESS--> qa\n\
         avril: qa --BLESS--> cto\n\
         avril: cto --BLESS--> stop\n"
    );
}

#[test]
fn committed_avril_bad_walk_names_po_and_the_verdicts() {
    // C-07 without the shell: `next` then `fail` at the po adversary.
    let g = committed("avril");
    let err = walk(&g, &[Event::Next, label("fail")]).expect_err("po accepts a verdict only");
    let msg = err.to_string();
    assert!(msg.contains("po"), "{msg}");
    assert!(msg.contains("BLESS, REJECT"), "{msg}");
    assert_eq!(err.trace().to_string(), "avril: generator --next--> po\n");
}

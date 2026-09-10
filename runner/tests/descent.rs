//! Descent tests (decision 5): `flagship` reaches its sink through `avril`,
//! `axel` and `code-gan`; a walk that stops inside `code-gan` is incomplete
//! in the innermost frame; `--flat` treats a `graph` node as opaque; a
//! directory whose `uses.graph` has no file is refused at load time.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::json;

use graph_runner::event::{Event, Verdict};
use graph_runner::graph::{Label, NodeId};
use graph_runner::load::{self, Graphs, LoadError};
use graph_runner::step::{self, StepError, WalkError};
use graph_runner::trace::Step;

fn graphs_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("graphs")
        .canonicalize()
        .expect("runner/ lives one level below the repo root")
}

fn committed() -> Graphs {
    load::load_dir(&graphs_dir()).expect("graphs/ loads")
}

fn id(s: &str) -> NodeId {
    NodeId::new(s)
}

fn label(s: &str) -> Event {
    Event::Label(Label::new(s))
}

const NEXT: Event = Event::Next;
const BLESS: Event = Event::Verdict(Verdict::Bless);

/// The flagship happy path, event by event, with the frame each belongs to.
fn flagship_happy() -> Vec<Event> {
    let mut events = vec![NEXT]; // flagship: intent → avril
    events.extend([NEXT, BLESS, BLESS, BLESS]); // avril: generator → po → qa → cto → stop
    events.push(label("blessed-backlog")); // flagship: avril → axel
    events.extend([label("blessed"), NEXT, label("pass"), BLESS]); // axel: intake → … → code-gan
    events.extend([NEXT, label("pass"), BLESS, BLESS]); // code-gan: generate → … → commit
    events.extend([BLESS, label("evidenced")]); // axel: code-gan → ac-evidence → done
    events.push(label("ac-evidenced")); // flagship: axel → done
    events
}

#[test]
fn flagship_reaches_its_sink_through_avril_axel_and_code_gan() {
    let graphs = committed();
    let flagship = &graphs["flagship"];
    let trace = step::walk(flagship, &graphs, &flagship_happy(), false)
        .unwrap_or_else(|e| panic!("{e}\n{}", e.trace()));
    assert!(trace.is_complete());
    assert_eq!(trace.at(), &id("done"));
    assert_eq!(
        trace.to_string(),
        "flagship: intent --next--> avril\n\
         flagship: avril >> avril\n\
         \x20 avril: generator --next--> po\n\
         \x20 avril: po --BLESS--> qa\n\
         \x20 avril: qa --BLESS--> cto\n\
         \x20 avril: cto --BLESS--> stop\n\
         \x20 avril: stop << flagship:avril\n\
         flagship: avril --blessed-backlog--> axel\n\
         flagship: axel >> axel\n\
         \x20 axel: intake --blessed--> plan-write\n\
         \x20 axel: plan-write --next--> plan-audit\n\
         \x20 axel: plan-audit --pass--> plan-architect\n\
         \x20 axel: plan-architect --BLESS--> code-gan\n\
         \x20 axel: code-gan >> code-gan\n\
         \x20   code-gan: generate --next--> mechanical\n\
         \x20   code-gan: mechanical --pass--> tester\n\
         \x20   code-gan: tester --BLESS--> reviewer\n\
         \x20   code-gan: reviewer --BLESS--> commit\n\
         \x20   code-gan: commit << axel:code-gan\n\
         \x20 axel: code-gan --BLESS--> ac-evidence\n\
         \x20 axel: ac-evidence --evidenced--> done\n\
         \x20 axel: done << flagship:axel\n\
         flagship: axel --ac-evidenced--> done\n"
    );

    let depth_of = |graph: &str| -> Vec<usize> {
        trace
            .steps
            .iter()
            .filter(|s| matches!(s, Step::Edge { graph: g, .. } if g == graph))
            .map(Step::depth)
            .collect()
    };
    assert!(depth_of("flagship").iter().all(|&d| d == 0));
    assert!(depth_of("avril").iter().all(|&d| d == 1));
    assert!(depth_of("axel").iter().all(|&d| d == 1));
    assert!(depth_of("code-gan").iter().all(|&d| d == 2));
    assert_eq!(depth_of("code-gan").len(), 4);
}

#[test]
fn a_walk_that_stops_inside_code_gan_is_incomplete_in_the_innermost_frame() {
    let graphs = committed();
    let flagship = &graphs["flagship"];
    // Stop after code-gan's reviewer blesses: the walk stands on
    // `code-gan:commit`, a sink of code-gan, two frames below flagship.
    let events: Vec<Event> = flagship_happy()[..14].to_vec();
    let err = step::walk(flagship, &graphs, &events, false).expect_err("not at depth 0");
    match &err {
        WalkError::Incomplete {
            graph,
            depth,
            trace,
            at,
        } => {
            assert_eq!(graph, "code-gan");
            assert_eq!(*depth, 2);
            assert_eq!(at, &id("commit"));
            assert_eq!(trace.at(), &id("commit"));
            assert!(
                trace
                    .to_string()
                    .ends_with("    code-gan: reviewer --BLESS--> commit\n"),
                "the return line is not printed: the frame is still open\n{trace}"
            );
        }
        other => panic!("expected Incomplete at code-gan:commit, got {other:?}"),
    }
    assert_eq!(err.to_string(), "incomplete at code-gan:commit");

    // One more event closes code-gan and steps axel; the walk still stands
    // below depth 0.
    let events: Vec<Event> = flagship_happy()[..15].to_vec();
    let err = step::walk(flagship, &graphs, &events, false).expect_err("axel is not done");
    assert_eq!(err.to_string(), "incomplete at axel:ac-evidence");
    assert!(err.trace().to_string().contains(
        "    code-gan: commit << axel:code-gan\n  axel: code-gan --BLESS--> ac-evidence\n"
    ));
}

#[test]
fn trailing_events_after_the_root_sink_are_still_an_error() {
    let graphs = committed();
    let flagship = &graphs["flagship"];
    let mut events = flagship_happy();
    events.push(NEXT);
    let err = step::walk(flagship, &graphs, &events, false).expect_err("done is the root sink");
    assert_eq!(
        err.to_string(),
        "trailing events after sink flagship:done: next"
    );
}

#[test]
fn flat_treats_a_graph_node_as_opaque() {
    let graphs = committed();
    let axel = &graphs["axel"];
    // Without descent, `code-gan` is a node with one BLESS out-edge.
    let flat_events = [
        label("blessed"),
        NEXT,
        label("pass"),
        BLESS,
        BLESS,
        label("evidenced"),
    ];
    let trace = step::walk(axel, &graphs, &flat_events, true).expect("flat walk completes");
    assert!(trace.is_complete());
    assert!(trace.steps.iter().all(|s| s.depth() == 0));
    assert!(trace.steps.iter().all(|s| matches!(s, Step::Edge { .. })));
    assert_eq!(
        trace.to_string(),
        "axel: intake --blessed--> plan-write\n\
         axel: plan-write --next--> plan-audit\n\
         axel: plan-audit --pass--> plan-architect\n\
         axel: plan-architect --BLESS--> code-gan\n\
         axel: code-gan --BLESS--> ac-evidence\n\
         axel: ac-evidence --evidenced--> done\n"
    );

    // With descent, the same events enter code-gan, where `generate` does
    // not accept BLESS.
    let err = step::walk(axel, &graphs, &flat_events, false).expect_err("BLESS is not next");
    assert_eq!(
        err.to_string(),
        "code-gan: generate accepts next, unsatisfiable-claim; got BLESS"
    );
    assert!(err
        .trace()
        .to_string()
        .ends_with("axel: code-gan >> code-gan\n"));

    // Flat, flagship's `avril` node has no out-edge for `next`.
    let flagship = &graphs["flagship"];
    let err = step::walk(flagship, &graphs, &[NEXT, NEXT], true).expect_err("avril is opaque");
    assert_eq!(
        err.to_string(),
        "flagship: avril accepts blessed-backlog; got next"
    );
}

#[test]
fn a_walk_may_re_enter_a_subgraph() {
    let graphs = committed();
    let axel = &graphs["axel"];
    let code_gan_happy = [NEXT, label("pass"), BLESS, BLESS];
    let mut events = vec![label("blessed"), NEXT, label("pass"), BLESS];
    events.extend(code_gan_happy.clone());
    events.extend([BLESS, label("missing-evidence")]);
    events.extend(code_gan_happy);
    events.extend([BLESS, label("evidenced")]);
    let trace = step::walk(axel, &graphs, &events, false).expect("two passes through code-gan");
    assert!(trace.is_complete());
    let descents = trace
        .steps
        .iter()
        .filter(|s| matches!(s, Step::Descend { sub, .. } if sub == "code-gan"))
        .count();
    assert_eq!(descents, 2);
    assert!(trace.to_string().contains(
        "  code-gan: commit << axel:code-gan\n\
         axel: code-gan --BLESS--> ac-evidence\n\
         axel: ac-evidence --missing-evidence--> code-gan\n\
         axel: code-gan >> code-gan\n\
         \x20 code-gan: generate --next--> mechanical\n"
    ));
}

fn write_dir(name: &str, files: &[(&str, serde_json::Value)]) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    fs::create_dir_all(&dir).expect("tmp dir");
    for (file, value) in files {
        fs::write(dir.join(format!("{file}.json")), value.to_string()).expect("write graph");
    }
    dir
}

fn outer(sub: &str) -> serde_json::Value {
    json!({
        "apiVersion": "crossr-loops/v0",
        "kind": "Graph",
        "name": "outer",
        "start": "in",
        "nodes": [
            {"id": "in", "role": "gate"},
            {"id": "sub", "role": "graph", "uses": {"graph": sub}},
            {"id": "out", "role": "terminal"}
        ],
        "edges": [
            {"from": "in", "to": "sub"},
            {"from": "sub", "to": "out", "when": "done"}
        ]
    })
}

#[test]
fn load_dir_refuses_a_subgraph_with_no_file() {
    let dir = write_dir("missing-subgraph", &[("outer", outer("inner"))]);
    let err = load::load_dir(&dir).expect_err("inner.json is not there");
    assert!(
        matches!(
            err,
            LoadError::MissingSubgraph { ref graph, ref node, ref sub }
                if graph == "outer" && node.as_str() == "sub" && sub == "inner"
        ),
        "{err}"
    );
    assert_eq!(
        err.to_string(),
        "outer: node 'sub' uses.graph 'inner' has no inner.json in the directory"
    );
}

#[test]
fn a_resolver_without_the_subgraph_fails_the_step_not_the_load() {
    let g = load::parse("outer", &outer("inner").to_string()).expect("outer parses alone");
    let err = step::walk(&g, &Graphs::new(), &[NEXT], false).expect_err("nothing resolves inner");
    match &err {
        WalkError::Step { trace, source } => {
            assert_eq!(
                source,
                &StepError::UnresolvedSubgraph {
                    graph: "outer".to_owned(),
                    node: id("sub"),
                    sub: "inner".to_owned(),
                }
            );
            assert_eq!(trace.to_string(), "outer: in --next--> sub\n");
        }
        other => panic!("expected Step/UnresolvedSubgraph, got {other:?}"),
    }
    assert_eq!(
        err.to_string(),
        "outer: sub descends into 'inner', which is not loaded"
    );
    // Flat never asks the resolver.
    let trace = step::walk(&g, &Graphs::new(), &[NEXT, label("done")], true).expect("opaque");
    assert!(trace.is_complete());
}

#[test]
fn descent_into_an_open_graph_is_refused() {
    // `outer` opens `inner`, whose start opens `outer` again: no event could
    // ever be consumed. The loader allows the pair (neither names itself).
    let inner = json!({
        "apiVersion": "crossr-loops/v0",
        "kind": "Graph",
        "name": "inner",
        "start": "back",
        "nodes": [
            {"id": "back", "role": "graph", "uses": {"graph": "outer"}},
            {"id": "stop", "role": "terminal"}
        ],
        "edges": [{"from": "back", "to": "stop"}]
    });
    let dir = write_dir("recursive", &[("outer", outer("inner")), ("inner", inner)]);
    let graphs = load::load_dir(&dir).expect("mutual reference loads");
    let err = step::walk(&graphs["outer"], &graphs, &[NEXT], false).expect_err("recursive");
    assert_eq!(
        err.to_string(),
        "inner: back descends into 'outer', which is already open"
    );
    assert_eq!(
        err.trace().to_string(),
        "outer: in --next--> sub\nouter: sub >> inner\n"
    );
}

//! `committed_walks`: every `graphs/walks/*.walk` whose graph has no
//! `role: graph` node replays to a sink. Descent is R3; a `graph` node is
//! opaque here, so walks through `axel` / `flagship` are not yet committed.

use std::fs;
use std::path::{Path, PathBuf};

use graph_runner::graph::Role;
use graph_runner::load;
use graph_runner::{step, walkfile};

fn graphs_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("graphs")
        .canonicalize()
        .expect("runner/ lives one level below the repo root")
}

fn walk_files() -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = fs::read_dir(graphs_dir().join("walks"))
        .expect("graphs/walks/ exists")
        .map(|entry| entry.expect("dir entry").path())
        .filter(|p| walkfile::is_walk_file(p))
        .collect();
    paths.sort();
    paths
}

#[test]
fn committed_walks() {
    let mut replayed: Vec<String> = Vec::new();
    for path in walk_files() {
        let file = path
            .file_name()
            .and_then(|n| n.to_str())
            .expect("utf-8 file name");
        let graph_name = walkfile::graph_name(&path).expect("walk file names its graph");
        let graph = load::load_file(&graphs_dir().join(format!("{graph_name}.json")))
            .unwrap_or_else(|e| panic!("{file}: graph {graph_name} loads: {e}"));
        if graph.nodes.iter().any(|n| n.role == Some(Role::Graph)) {
            continue;
        }

        let text = fs::read_to_string(&path).expect("walk file readable");
        assert!(
            text.lines().next().is_some_and(|l| l.starts_with('#')),
            "{file}: first line must be a `#` story line"
        );
        let events = walkfile::parse(&text).unwrap_or_else(|e| panic!("{file}: {e}"));
        let trace = step::walk(&graph, &events)
            .unwrap_or_else(|e| panic!("{file}: does not reach a sink: {e}\n{}", e.trace()));
        assert!(trace.is_complete(), "{file}: {trace}");
        assert!(
            step::is_sink(&graph, trace.at()),
            "{file}: ends on {} which is not a sink",
            trace.at()
        );
        replayed.push(file.trim_end_matches(".walk").to_owned());
    }
    assert_eq!(
        replayed,
        ["avril.happy", "brick.happy", "code-gan.happy"],
        "R2 commits the three descent-free happy paths"
    );
}

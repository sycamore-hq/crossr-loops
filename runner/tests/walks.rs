//! `committed_walks`: every `graphs/walks/*.walk` replays to a sink with
//! descent on, and together they take every edge of every committed graph
//! (decision 6). `cargo test` is red while `cover` reports an uncovered edge.

use std::fs;
use std::path::{Path, PathBuf};

use graph_runner::event::Event;
use graph_runner::{cover, load, step, walkfile};

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
    let graphs = load::load_dir(&graphs_dir()).expect("graphs/ loads");
    let mut replayed: Vec<String> = Vec::new();
    let mut walks: Vec<(String, Vec<Event>)> = Vec::new();
    for path in walk_files() {
        let file = path
            .file_name()
            .and_then(|n| n.to_str())
            .expect("utf-8 file name");
        let graph_name = walkfile::graph_name(&path).expect("walk file names its graph");
        let graph = graphs
            .get(graph_name)
            .unwrap_or_else(|| panic!("{file}: no graphs/{graph_name}.json"));

        let text = fs::read_to_string(&path).expect("walk file readable");
        assert!(
            text.lines().next().is_some_and(|l| l.starts_with('#')),
            "{file}: first line must be a `#` story line"
        );
        let events = walkfile::parse(&text).unwrap_or_else(|e| panic!("{file}: {e}"));
        let trace = step::walk(graph, &graphs, &events, false)
            .unwrap_or_else(|e| panic!("{file}: does not reach a sink: {e}\n{}", e.trace()));
        assert!(trace.is_complete(), "{file}: {trace}");
        assert!(
            step::is_sink(graph, trace.at()),
            "{file}: ends on {} which is not a sink",
            trace.at()
        );
        replayed.push(file.trim_end_matches(".walk").to_owned());
        walks.push((graph_name.to_owned(), events));
    }

    for required in [
        "avril.happy",
        "axel.happy",
        "brick.happy",
        "code-gan.happy",
        "flagship.happy",
    ] {
        assert!(
            replayed.iter().any(|r| r == required),
            "{required}.walk is committed; replayed {replayed:?}"
        );
    }

    let coverage = cover::cover(&graphs, &walks).expect("every committed walk completes");
    let total: usize = graphs.values().map(|g| g.edges.len()).sum();
    assert_eq!(coverage.total, total);
    assert!(
        coverage.uncovered.is_empty(),
        "{} walks leave {} of {total} edges untaken:\n{}",
        walks.len(),
        coverage.uncovered.len(),
        coverage
            .uncovered
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert_eq!(coverage.taken.len(), total);
}

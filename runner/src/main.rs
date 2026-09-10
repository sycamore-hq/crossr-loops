//! `graph-runner check <dir>` — load every graph in `<dir>` and report.
//! `graph-runner walk [--flat] <graph.json> <walk-file>` — replay a walk,
//! descending into subgraphs found beside `<graph.json>`, and print the path.
//! `graph-runner cover <dir>` — replay every `<dir>/walks/*.walk` and report
//! the edges no walk took.
//!
//! Actions only: argv, file reads, stdout, stderr, exit code. Exit 0 all
//! graphs load / the walk completes / every edge is covered; 1 a graph was
//! refused, the walk did not complete (trace so far on stdout, the error on
//! stderr), or an edge is uncovered; 2 bad argv.

use std::ffi::OsStr;
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use graph_runner::event::Event;
use graph_runner::graph::Graph;
use graph_runner::load::{self, Graphs, LoadError};
use graph_runner::{cover, step, walkfile};

const USAGE: &str = "usage: graph-runner check <dir>\n       \
     graph-runner walk [--flat] <graph.json> <walk-file>\n       \
     graph-runner cover <dir>";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [cmd, dir] if cmd == "check" => match check(Path::new(dir)) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => fail(&e),
        },
        [cmd, graph, walk] if cmd == "walk" => walk_cmd(Path::new(graph), Path::new(walk), false),
        [cmd, flag, graph, walk] if cmd == "walk" && flag == "--flat" => {
            walk_cmd(Path::new(graph), Path::new(walk), true)
        }
        [cmd, dir] if cmd == "cover" => cover_cmd(Path::new(dir)),
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn fail(e: &dyn Display) -> ExitCode {
    eprintln!("✗ {e}");
    ExitCode::from(1)
}

fn check(dir: &Path) -> Result<(), LoadError> {
    let graphs = load::load_dir(dir)?;
    for g in graphs.values() {
        println!("{}", summary(g));
    }
    println!("✓ {} graphs OK", graphs.len());
    Ok(())
}

fn summary(g: &Graph) -> String {
    let sinks = g
        .sinks()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "  ✓ {}: {} nodes, {} edges, start {}, sinks [{sinks}]",
        g.name,
        g.nodes.len(),
        g.edges.len(),
        g.start
    )
}

/// Read one walk file into events, naming the file in any error.
fn read_walk(path: &Path) -> Result<Vec<Event>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    walkfile::parse(&text).map_err(|e| format!("{}: {e}", path.display()))
}

/// Load every graph beside `graph_path` and name the one it points at.
fn load_beside(graph_path: &Path) -> Result<(Graphs, String), String> {
    let dir = graph_path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let graphs = load::load_dir(dir).map_err(|e| e.to_string())?;
    let stem = graph_path
        .file_stem()
        .and_then(OsStr::to_str)
        .filter(|stem| graphs.contains_key(*stem))
        .ok_or_else(|| format!("{}: not a graph in {}", graph_path.display(), dir.display()))?;
    Ok((graphs, stem.to_owned()))
}

fn walk_cmd(graph_path: &Path, walk_path: &Path, flat: bool) -> ExitCode {
    let (graphs, stem) = match load_beside(graph_path) {
        Ok(loaded) => loaded,
        Err(e) => return fail(&e),
    };
    let Some(graph) = graphs.get(&stem) else {
        return fail(&format!("{}: not loaded", graph_path.display()));
    };
    let events = match read_walk(walk_path) {
        Ok(events) => events,
        Err(e) => return fail(&e),
    };
    match step::walk(graph, &graphs, &events, flat) {
        Ok(trace) => {
            print!("{trace}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            print!("{}", e.trace());
            fail(&e)
        }
    }
}

/// Every `*.walk` under `<dir>/walks`, in file-name order.
fn walk_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let walks = dir.join("walks");
    let io = |e| format!("{}: {e}", walks.display());
    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(&walks).map_err(io)? {
        let path = entry.map_err(io)?.path();
        if walkfile::is_walk_file(&path) {
            paths.push(path);
        }
    }
    paths.sort();
    Ok(paths)
}

/// `(graph, events)` for every walk file, naming the file in any error.
fn read_walks(paths: &[PathBuf]) -> Result<Vec<(String, Vec<Event>)>, String> {
    paths
        .iter()
        .map(|path| {
            let graph = walkfile::graph_name(path)
                .ok_or_else(|| format!("{}: file name does not name a graph", path.display()))?;
            Ok((graph.to_owned(), read_walk(path)?))
        })
        .collect()
}

fn cover_cmd(dir: &Path) -> ExitCode {
    let graphs = match load::load_dir(dir) {
        Ok(graphs) => graphs,
        Err(e) => return fail(&e),
    };
    let paths = match walk_files(dir) {
        Ok(paths) => paths,
        Err(e) => return fail(&e),
    };
    let walks = match read_walks(&paths) {
        Ok(walks) => walks,
        Err(e) => return fail(&e),
    };
    let coverage = match cover::cover(&graphs, &walks) {
        Ok(coverage) => coverage,
        Err(e) => {
            let (walk, source) = match &e {
                cover::CoverError::UnknownGraph { walk, .. } => (*walk, None),
                cover::CoverError::Walk { walk, source, .. } => (*walk, Some(source)),
            };
            if let Some(source) = source {
                print!("{}", source.trace());
            }
            let file = paths
                .get(walk)
                .map_or_else(String::new, |p| p.display().to_string());
            return fail(&format!("{file}: {e}"));
        }
    };
    println!("taken {}/{} edges", coverage.taken.len(), coverage.total);
    for edge in &coverage.uncovered {
        println!("{edge}");
    }
    println!("uncovered edges: {}", coverage.uncovered.len());
    if coverage.is_full() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

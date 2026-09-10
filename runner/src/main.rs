//! `graph-runner check <dir>` — load every graph in `<dir>` and report.
//! `graph-runner walk <graph.json> <walk-file>` — replay a walk and print
//! the path.
//!
//! Actions only: argv, file reads, stdout, stderr, exit code. Exit 0 all
//! graphs load / the walk completes; 1 a graph was refused or the walk did
//! not complete (trace so far on stdout, the error on stderr); 2 bad argv.

use std::fmt::Display;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use graph_runner::graph::Graph;
use graph_runner::load::{self, LoadError};
use graph_runner::{step, walkfile};

const USAGE: &str =
    "usage: graph-runner check <dir>\n       graph-runner walk <graph.json> <walk-file>";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [cmd, dir] if cmd == "check" => match check(Path::new(dir)) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => fail(&e),
        },
        [cmd, graph, walk] if cmd == "walk" => walk_cmd(Path::new(graph), Path::new(walk)),
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
    for g in &graphs {
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

fn walk_cmd(graph_path: &Path, walk_path: &Path) -> ExitCode {
    let graph = match load::load_file(graph_path) {
        Ok(g) => g,
        Err(e) => return fail(&e),
    };
    let text = match fs::read_to_string(walk_path) {
        Ok(text) => text,
        Err(e) => return fail(&format!("{}: {e}", walk_path.display())),
    };
    let events = match walkfile::parse(&text) {
        Ok(events) => events,
        Err(e) => return fail(&format!("{}: {e}", walk_path.display())),
    };
    match step::walk(&graph, &events) {
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

//! `graph-runner check <dir>` — load every graph in `<dir>` and report.
//!
//! Actions only: argv, file reads (through [`load`]), stdout, stderr, exit
//! code. Exit 0 all graphs load; 1 a graph was refused; 2 bad argv.

use std::path::Path;
use std::process::ExitCode;

use graph_runner::graph::Graph;
use graph_runner::load::{self, LoadError};

const USAGE: &str = "usage: graph-runner check <dir>";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [cmd, dir] if cmd == "check" => match check(Path::new(dir)) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("✗ {e}");
                ExitCode::from(1)
            }
        },
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
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

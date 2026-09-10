//! `graph-runner`: a typed reader and replayer for crossr-loops
//! `graphs/*.json`.
//!
//! It reads the graph files and nothing else: no skill file, no model, no
//! process, no socket. Events come from the caller; the runner never decides
//! a verdict. It replays a walk against the topology and never runs one.
//! The conductor skill wins over a graph; the graph wins over this runner.

pub mod cover;
pub mod event;
pub mod graph;
pub mod load;
pub mod step;
pub mod trace;
pub mod walkfile;

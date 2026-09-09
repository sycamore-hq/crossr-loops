//! `graph-runner`: a typed reader for crossr-loops `graphs/*.json`.
//!
//! It reads the graph files and nothing else: no skill file, no model, no
//! process, no socket. The conductor skill wins over a graph; the graph wins
//! over this reader.

pub mod graph;
pub mod load;

//! Loader tests: the six C-02 refusals, the C-03 schema/enum agreement, and
//! the remaining `check` rules against fixtures and the committed graphs.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use graph_runner::graph::{Graph, Label, NodeId, Role};
use graph_runner::load::{self, EdgeEnd, LoadError};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .canonicalize()
        .expect("runner/ lives one level below the repo root")
}

fn graphs_dir() -> PathBuf {
    repo_root().join("graphs")
}

/// A minimal schema-shaped graph: generator → adversary → terminal.
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

fn parse(v: &Value) -> Result<Graph, LoadError> {
    load::parse("fixture", &v.to_string())
}

fn refuse(v: &Value) -> LoadError {
    parse(v).expect_err("fixture variant must be refused")
}

// ---- C-02 -----------------------------------------------------------------

#[test]
fn load_rejects_unknown_key() {
    let mut v = fixture();
    v["extra"] = json!(1);
    let err = refuse(&v);
    assert!(matches!(err, LoadError::Json { .. }), "{err}");
    let msg = err.to_string();
    assert!(msg.starts_with("fixture: "), "{msg}");
    assert!(msg.contains("unknown field `extra`"), "{msg}");
}

#[test]
fn load_rejects_unknown_role() {
    let mut v = fixture();
    v["nodes"][0]["role"] = json!("wizard");
    let err = refuse(&v);
    assert!(matches!(err, LoadError::Json { .. }), "{err}");
    let msg = err.to_string();
    assert!(msg.contains("unknown variant `wizard`"), "{msg}");
    assert!(msg.contains("`generator`"), "{msg}");
}

#[test]
fn load_rejects_dangling_edge() {
    let mut v = fixture();
    v["edges"]
        .as_array_mut()
        .expect("edges is an array")
        .push(json!({"from": "stop", "to": "nowhere"}));
    let err = refuse(&v);
    assert!(
        matches!(
            err,
            LoadError::DanglingEdge { ref graph, index: 3, end: EdgeEnd::To, ref node }
                if graph == "fixture" && node.as_str() == "nowhere"
        ),
        "{err}"
    );
    assert_eq!(
        err.to_string(),
        "fixture: edge 3 to 'nowhere' is not a node"
    );
}

#[test]
fn load_rejects_ambiguous_out_edges() {
    let mut v = fixture();
    v["edges"]
        .as_array_mut()
        .expect("edges is an array")
        .push(json!({"from": "b", "to": "a", "when": "BLESS"}));
    let err = refuse(&v);
    assert!(
        matches!(
            err,
            LoadError::AmbiguousOutEdges { ref node, label: Some(ref l), .. }
                if node.as_str() == "b" && l.as_str() == "BLESS"
        ),
        "{err}"
    );
    assert_eq!(
        err.to_string(),
        "fixture: node 'b' has two out-edges with when 'BLESS'"
    );
}

#[test]
fn load_rejects_reserved_next() {
    let mut v = fixture();
    v["edges"][1]["when"] = json!(Label::NEXT);
    let err = refuse(&v);
    assert!(
        matches!(
            err,
            LoadError::ReservedLabel { index: 1, ref from, ref to, .. }
                if from.as_str() == "b" && to.as_str() == "stop"
        ),
        "{err}"
    );
    assert_eq!(
        err.to_string(),
        "fixture: edge 1 b -> stop uses the reserved label 'next'"
    );
}

#[test]
fn load_rejects_start_naming_no_node() {
    let mut v = fixture();
    v["start"] = json!("nope");
    let err = refuse(&v);
    assert!(
        matches!(
            err,
            LoadError::StartNotANode { ref graph, ref start }
                if graph == "fixture" && start.as_str() == "nope"
        ),
        "{err}"
    );
    assert_eq!(err.to_string(), "fixture: start 'nope' is not a node");
}

// ---- C-03 -----------------------------------------------------------------

#[test]
fn role_enum_matches_schema() {
    let text = fs::read_to_string(graphs_dir().join("schema.json")).expect("schema.json readable");
    let schema: Value = serde_json::from_str(&text).expect("schema.json parses");
    let from_schema = &schema["properties"]["nodes"]["items"]["properties"]["role"]["enum"];
    assert!(
        from_schema.is_array(),
        "schema role enum missing: {from_schema}"
    );

    let from_rust = serde_json::to_value(Role::ALL).expect("Role serializes");
    assert_eq!(
        &from_rust, from_schema,
        "Role::ALL must equal the schema enum, in order"
    );

    let displayed: Vec<Value> = Role::ALL.iter().map(|r| json!(r.to_string())).collect();
    assert_eq!(
        Value::Array(displayed),
        *from_schema,
        "Display must spell roles as the schema does"
    );
}

// ---- the committed graphs -----------------------------------------------

#[test]
fn committed_graphs_all_check() {
    let graphs = load::load_dir(&graphs_dir()).expect("graphs/ loads");
    let names: Vec<&str> = graphs.iter().map(|g| g.name.as_str()).collect();
    assert_eq!(names, ["avril", "axel", "brick", "code-gan", "flagship"]);
    for g in &graphs {
        assert_eq!(g.api_version, Graph::API_VERSION);
        assert_eq!(g.kind, Graph::KIND);
        assert!(g.node(&g.start).is_some(), "{}: start is a node", g.name);
    }
}

#[test]
fn sinks_are_nodes_without_out_edges() {
    let dir = graphs_dir();
    let avril = load::load_file(&dir.join("avril.json")).expect("avril loads");
    assert_eq!(avril.sinks(), [&NodeId::new("stop")]);
    let code_gan = load::load_file(&dir.join("code-gan.json")).expect("code-gan loads");
    assert_eq!(code_gan.sinks(), [&NodeId::new("commit")]);
    assert_eq!(
        code_gan.node(&NodeId::new("commit")).and_then(|n| n.role),
        Some(Role::Gate)
    );
}

#[test]
fn schema_file_is_skipped_and_empty_dir_is_refused() {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("only-schema");
    fs::create_dir_all(&dir).expect("tmp dir");
    fs::copy(graphs_dir().join("schema.json"), dir.join("schema.json")).expect("copy schema");
    let err = load::load_dir(&dir).expect_err("a dir with only schema.json is refused");
    assert!(matches!(err, LoadError::NoGraphs { .. }), "{err}");
}

// ---- the remaining check rules --------------------------------------------

#[test]
fn fixture_is_accepted() {
    let g = parse(&fixture()).expect("fixture loads");
    assert_eq!(g.start, NodeId::new("a"));
    assert_eq!(g.sinks(), [&NodeId::new("stop")]);
    assert_eq!(g.out_edges(&NodeId::new("b")).count(), 2);
    assert_eq!(
        g.node(&NodeId::new("b")).and_then(|n| n.role),
        Some(Role::Adversary)
    );
}

#[test]
fn check_rejects_missing_required_key() {
    let mut v = fixture();
    v.as_object_mut().expect("object").remove("start");
    let err = refuse(&v);
    assert!(matches!(err, LoadError::Json { .. }), "{err}");
    assert!(err.to_string().contains("missing field `start`"), "{err}");
}

#[test]
fn check_rejects_unknown_uses_key() {
    let mut v = fixture();
    v["nodes"][1]["uses"]["model"] = json!("gpt");
    let err = refuse(&v);
    assert!(matches!(err, LoadError::Json { .. }), "{err}");
    assert!(err.to_string().contains("unknown field `model`"), "{err}");
}

#[test]
fn check_rejects_api_version() {
    let mut v = fixture();
    v["apiVersion"] = json!("crossr-loops/v1");
    let err = refuse(&v);
    assert!(
        matches!(err, LoadError::ApiVersion { ref found, .. } if found == "crossr-loops/v1"),
        "{err}"
    );
}

#[test]
fn check_rejects_kind() {
    let mut v = fixture();
    v["kind"] = json!("Pipeline");
    let err = refuse(&v);
    assert!(
        matches!(err, LoadError::Kind { ref found, .. } if found == "Pipeline"),
        "{err}"
    );
}

#[test]
fn check_rejects_empty_nodes() {
    let mut v = fixture();
    v["nodes"] = json!([]);
    v["edges"] = json!([]);
    let err = refuse(&v);
    assert!(
        matches!(err, LoadError::NoNodes { ref graph } if graph == "fixture"),
        "{err}"
    );
}

#[test]
fn check_rejects_duplicate_node() {
    let mut v = fixture();
    v["nodes"]
        .as_array_mut()
        .expect("nodes is an array")
        .push(json!({"id": "a"}));
    let err = refuse(&v);
    assert!(
        matches!(err, LoadError::DuplicateNode { ref node, .. } if node.as_str() == "a"),
        "{err}"
    );
}

#[test]
fn check_rejects_two_unlabeled_out_edges() {
    let mut v = fixture();
    v["edges"]
        .as_array_mut()
        .expect("edges is an array")
        .push(json!({"from": "a", "to": "stop"}));
    let err = refuse(&v);
    assert!(
        matches!(err, LoadError::AmbiguousOutEdges { ref node, label: None, .. } if node.as_str() == "a"),
        "{err}"
    );
    assert_eq!(
        err.to_string(),
        "fixture: node 'a' has two out-edges without when (both fire on 'next')"
    );
}

#[test]
fn check_rejects_exclusive_uses() {
    let mut v = fixture();
    v["nodes"][0]["uses"] = json!({"skill": "code-writer", "graph": "code-gan"});
    let err = refuse(&v);
    assert!(
        matches!(err, LoadError::ExclusiveUses { ref node, .. } if node.as_str() == "a"),
        "{err}"
    );
}

#[test]
fn check_rejects_self_reference() {
    let mut v = fixture();
    v["nodes"][0]["role"] = json!("graph");
    v["nodes"][0]["uses"] = json!({"graph": "fixture"});
    let err = refuse(&v);
    assert!(
        matches!(err, LoadError::SelfReference { ref node, .. } if node.as_str() == "a"),
        "{err}"
    );
}

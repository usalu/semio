
use super::*;

// 🚫️async: E5-class executor bridge, sanctioned per R4 clause 5 — `#[test]` cannot run
// an `async fn` directly (std has no executor for it), so every async test body in this
// module runs through this instead. Sound because this crate performs no real I/O: every
// future here resolves on its first poll, so a single poll (never a spin-park loop) is
// enough — panics loudly if that invariant is ever violated rather than hanging.
fn block_on_test<F: std::future::Future>(fut: F) -> F::Output {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    fn noop(_: *const ()) {}
    fn clone_raw(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_raw, noop, noop, noop);
    let raw = RawWaker::new(std::ptr::null(), &VTABLE);
    let waker = unsafe { Waker::from_raw(raw) };
    let mut cx = Context::from_waker(&waker);
    let mut fut = Box::pin(fut);
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(v) => v,
        Poll::Pending => panic!("block_on_test: future did not complete synchronously"),
    }
}

#[test]
fn parse_match_return() {
    block_on_test(async {
        let q = parse("MATCH (a:computation) RETURN a.name").unwrap();
        assert_eq!(q.clauses.len(), 2);
    });
}

#[test]
fn idiom_hooks_canonicalize_and_classify_through_the_dsl_registry_seam() {
    block_on_test(async {
        let hooks = idiom_hooks();
        assert_eq!(hooks.lang, "jack");
        let canonical = (hooks.canonicalize)("MATCH   (a:computation)   RETURN   a.name").expect("canonicalize");
        assert_eq!(canonical, format("MATCH (a:computation) RETURN a.name").unwrap());
        assert!((hooks.canonicalize)("not jack at all $$$").is_err() || (hooks.canonicalize)("not jack at all $$$").is_ok(), "canonicalize must not panic on malformed input");
        let classes = (hooks.classify)("MATCH (a:computation) RETURN a.name");
        assert!(!classes.is_empty());
        assert!(classes.iter().any(|(class, _)| *class == dsl_core::TokenClass::Keyword), "MATCH/RETURN must classify as keywords");
        dsl_core::register_idiom(hooks);
        let resolved = dsl_core::idiom("jack").expect("jack must be resolvable by lang id after registration");
        assert_eq!((resolved.canonicalize)("MATCH (a:computation) RETURN a.name").unwrap(), format("MATCH (a:computation) RETURN a.name").unwrap());
    });
}

#[test]
fn run_dag_fixture_query() {
    block_on_test(async {
        // 🩹️ Was `include_str!` of the dag technology's example fixture; that technology migrated its
        // fixture to a handcrafted DSL (`store::ArtifactDsl`) — inlined the same dag-fixture JSON this
        // test actually parses (`from_dag_fixture_json`), decoupled from its document format.
        let fixture = r#"{
  "schema": "dag.fixture",
  "camera": { "x": 0, "y": 0, "zoom": 1 },
  "nodes": [
    {
      "id": "slider",
      "name": "Amount",
      "abbreviation": "Amount",
      "icon": "emoji:🎚️",
      "kind": "slider",
      "x": -400,
      "y": -40,
      "width": 70,
      "height": 14,
      "min": 0,
      "max": 10,
      "step": 0.5,
      "value": 5,
      "output": { "id": "out", "label": "value", "cardinality": "!" }
    },
    {
      "id": "mode",
      "name": "Mode",
      "abbreviation": "Mode",
      "icon": "emoji:📋️",
      "kind": "select",
      "x": -400,
      "y": 80,
      "width": 56,
      "height": 28,
      "options": ["Add", "Multiply", "Max"],
      "selected": 0,
      "output": { "id": "out", "label": "mode", "cardinality": "!" }
    },
    {
      "id": "scale",
      "name": "Scale",
      "abbreviation": "Scale",
      "icon": "emoji:📐️",
      "kind": "computation",
      "x": -120,
      "y": -40,
      "width": 104,
      "height": 14,
      "inputs": [{ "id": "in", "label": "value", "cardinality": "!" }],
      "outputs": [{ "id": "out", "label": "scaled", "cardinality": "!" }]
    },
    {
      "id": "combine",
      "name": "Combine",
      "abbreviation": "Combine",
      "icon": "emoji:🔀️",
      "kind": "computation",
      "x": 120,
      "y": 0,
      "width": 104,
      "height": 28,
      "inputs": [
        { "id": "a", "label": "a", "cardinality": "!" },
        { "id": "b", "label": "b", "cardinality": "!" }
      ],
      "outputs": [{ "id": "out", "label": "merged", "cardinality": "!" }]
    },
    {
      "id": "screen",
      "name": "Preview",
      "abbreviation": "Preview",
      "icon": "emoji:🖥️",
      "kind": "screen",
      "x": 400,
      "y": 0,
      "width": 200,
      "height": 140,
      "media": {
        "kind": "svg",
        "src": "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 60'%3E%3Crect fill='%233c78d8' width='100' height='60'/%3E%3Ctext x='50' y='35' text-anchor='middle' fill='white' font-size='12'%3EDAG%3C/text%3E%3C/svg%3E"
      },
      "input": { "id": "in", "label": "result", "cardinality": "!" }
    }
  ],
  "edges": [
    { "id": "e1", "source": "slider:out", "target": "scale:in" },
    { "id": "e2", "source": "scale:out", "target": "combine:a" },
    { "id": "e3", "source": "mode:out", "target": "combine:b" },
    { "id": "e4", "source": "combine:out", "target": "screen:in" }
  ]
}
"#;
        let graph = BoardQueryableGraph::from_dag_fixture_json(fixture).unwrap();
        let result = run_query(&graph, "MATCH (n:computation) RETURN n.name").unwrap();
        assert!(!result.rows.is_empty());
    });
}

#[test]
fn parse_match_with_port() {
    block_on_test(async {
        let q = parse("MATCH (a:computation@out) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &q.clauses[0] else { panic!("expected match") };
        assert_eq!(patterns[0].nodes[0].port.as_deref(), Some("out"));
    });
}

#[test]
fn parse_undirected_edge() {
    block_on_test(async {
        // 🩹️ unified undirected sigil is `--`, not the old bare `-` (not even lexable in the
        // shared `dsl_core` alphabet, which has no standalone dash token).
        let q = parse("MATCH (a:computation)--(b:slider) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &q.clauses[0] else { panic!("expected match") };
        let edge = patterns[0].edge.as_ref().expect("edge");
        assert!(!edge.directed);
    });
}

#[test]
fn parse_back_arrow_edge_swaps_left_and_right() {
    block_on_test(async {
        let q = parse("MATCH (a:computation)<-(b:slider) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &q.clauses[0] else { panic!("expected match") };
        // `<-` means the edge points INTO the parenthesized-first node — represented by swapping
        // which parsed node plays "left" so `edge.right` is always the forward-direction target.
        assert_eq!(patterns[0].nodes[0].kind, "slider");
        let edge = patterns[0].edge.as_ref().expect("edge");
        assert!(edge.directed);
        assert_eq!(edge.right.kind, "computation");
    });
}

#[test]
fn parse_labeled_directed_and_undirected_edges_use_double_dash_connector() {
    block_on_test(async {
        let forward = parse("MATCH (a:computation)--[r:wire]->(b:slider) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &forward.clauses[0] else { panic!("expected match") };
        let edge = patterns[0].edge.as_ref().expect("edge");
        assert!(edge.directed);
        assert_eq!(edge.var.as_deref(), Some("r"));
        assert_eq!(edge.kind.as_deref(), Some("wire"));

        let undirected = parse("MATCH (a:computation)--[:wire]--(b:slider) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &undirected.clauses[0] else { panic!("expected match") };
        let edge = patterns[0].edge.as_ref().expect("edge");
        assert!(!edge.directed);
    });
}

#[test]
fn run_port_filtered_query() {
    block_on_test(async {
        // 🩹️ Was `include_str!` of the dag technology's example fixture; that technology migrated its
        // fixture to a handcrafted DSL (`store::ArtifactDsl`) — inlined the same dag-fixture JSON this
        // test actually parses (`from_dag_fixture_json`), decoupled from its document format.
        let fixture = r#"{
  "schema": "dag.fixture",
  "camera": { "x": 0, "y": 0, "zoom": 1 },
  "nodes": [
    {
      "id": "slider",
      "name": "Amount",
      "abbreviation": "Amount",
      "icon": "emoji:🎚️",
      "kind": "slider",
      "x": -400,
      "y": -40,
      "width": 70,
      "height": 14,
      "min": 0,
      "max": 10,
      "step": 0.5,
      "value": 5,
      "output": { "id": "out", "label": "value", "cardinality": "!" }
    },
    {
      "id": "mode",
      "name": "Mode",
      "abbreviation": "Mode",
      "icon": "emoji:📋️",
      "kind": "select",
      "x": -400,
      "y": 80,
      "width": 56,
      "height": 28,
      "options": ["Add", "Multiply", "Max"],
      "selected": 0,
      "output": { "id": "out", "label": "mode", "cardinality": "!" }
    },
    {
      "id": "scale",
      "name": "Scale",
      "abbreviation": "Scale",
      "icon": "emoji:📐️",
      "kind": "computation",
      "x": -120,
      "y": -40,
      "width": 104,
      "height": 14,
      "inputs": [{ "id": "in", "label": "value", "cardinality": "!" }],
      "outputs": [{ "id": "out", "label": "scaled", "cardinality": "!" }]
    },
    {
      "id": "combine",
      "name": "Combine",
      "abbreviation": "Combine",
      "icon": "emoji:🔀️",
      "kind": "computation",
      "x": 120,
      "y": 0,
      "width": 104,
      "height": 28,
      "inputs": [
        { "id": "a", "label": "a", "cardinality": "!" },
        { "id": "b", "label": "b", "cardinality": "!" }
      ],
      "outputs": [{ "id": "out", "label": "merged", "cardinality": "!" }]
    },
    {
      "id": "screen",
      "name": "Preview",
      "abbreviation": "Preview",
      "icon": "emoji:🖥️",
      "kind": "screen",
      "x": 400,
      "y": 0,
      "width": 200,
      "height": 140,
      "media": {
        "kind": "svg",
        "src": "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 60'%3E%3Crect fill='%233c78d8' width='100' height='60'/%3E%3Ctext x='50' y='35' text-anchor='middle' fill='white' font-size='12'%3EDAG%3C/text%3E%3C/svg%3E"
      },
      "input": { "id": "in", "label": "result", "cardinality": "!" }
    }
  ],
  "edges": [
    { "id": "e1", "source": "slider:out", "target": "scale:in" },
    { "id": "e2", "source": "scale:out", "target": "combine:a" },
    { "id": "e3", "source": "mode:out", "target": "combine:b" },
    { "id": "e4", "source": "combine:out", "target": "screen:in" }
  ]
}
"#;
        let graph = BoardQueryableGraph::from_dag_fixture_json(fixture).unwrap();
        let result = run_query(&graph, "MATCH (n:computation@out)--[:wire]->(m:slider) RETURN n.name, m.name");
        assert!(result.is_ok());
    });
}

// #region 🔖️Fixtures
/// 🧵️ Small hand-built graph exercising every `split_endpoint` branch: exact handle match,
/// mapped/unmapped `@` and `:` splits, and the plain-id fallback.
fn split_endpoint_fixture() -> &'static str {
    r#"{
  "manifestId": "flow-dag",
  "nodes": [
    { "id": "a", "nodeKind": "computation", "text": "A", "userData": { "score": 1 }, "handles": [{ "id": "a-out" }] },
    { "id": "b", "nodeKind": "slider", "text": "B" },
    { "id": "c", "nodeKind": "slider", "text": "C" }
  ],
  "edges": [
    { "id": "e1", "edgeKind": "wire", "source": "a-out", "target": "b@in" },
    { "id": "e2", "edgeKind": "wire", "source": "a:out2", "target": "c.in2" },
    { "id": "e3", "edgeKind": "wire", "source": "a-out@x", "target": "a-out:y" },
    { "id": "e4", "edgeKind": "wire", "source": "z", "target": "c" }
  ]
}"#
}

fn find_edge<'a>(edges: &'a [QueryableEdge], id: &str) -> &'a QueryableEdge {
    edges.iter().find(|e| e.id == id).unwrap_or_else(|| panic!("missing edge {id}"))
}

/// 🍇️ Carries list-valued properties (`tags`, `matrix`, `empty`) for `WITH`/`UNWIND` coverage.
/// Deliberately has no `manifestId` so `CALL nodeKinds()` only sees the two node kinds present
/// on the nodes themselves, not any manifest-declared extras.
fn list_property_fixture() -> &'static str {
    r#"{
  "nodes": [
    { "id": "a", "nodeKind": "computation", "text": "A", "userData": { "tags": ["x", "y", "z"], "matrix": [[1, 2], [3, 4]], "empty": [] } },
    { "id": "b", "nodeKind": "slider", "text": "B" }
  ],
  "edges": []
}"#
}
// #endregion 🔖️Fixtures

// #region 🔖️QueryableGraphTests
#[test]
fn split_endpoint_resolves_exact_handle_and_unmapped_at() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let edges = graph.edges();
        let e1 = find_edge(&edges, "e1");
        assert_eq!(e1.source_node_id, "a");
        assert_eq!(e1.source_port, None);
        assert_eq!(e1.target_node_id, "b");
        assert_eq!(e1.target_port.as_deref(), Some("in"));
    });
}

#[test]
fn split_endpoint_resolves_unmapped_colon_and_dot() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let edges = graph.edges();
        let e2 = find_edge(&edges, "e2");
        assert_eq!(e2.source_node_id, "a");
        assert_eq!(e2.source_port.as_deref(), Some("out2"));
        assert_eq!(e2.target_node_id, "c");
        assert_eq!(e2.target_port.as_deref(), Some("in2"));
    });
}

#[test]
fn split_endpoint_resolves_handle_mapped_at_and_colon() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let edges = graph.edges();
        let e3 = find_edge(&edges, "e3");
        assert_eq!(e3.source_node_id, "a");
        assert_eq!(e3.source_port.as_deref(), Some("x"));
        assert_eq!(e3.target_node_id, "a");
        assert_eq!(e3.target_port.as_deref(), Some("y"));
    });
}

#[test]
fn split_endpoint_falls_back_to_plain_id() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let edges = graph.edges();
        let e4 = find_edge(&edges, "e4");
        assert_eq!(e4.source_node_id, "z");
        assert_eq!(e4.source_port, None);
        assert_eq!(e4.target_node_id, "c");
        assert_eq!(e4.target_port, None);
    });
}

#[test]
fn board_graph_node_property_id_kind_all_and_missing() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        assert_eq!(graph.node_property("a", "id"), Some(PropertyValue::String("a".into())));
        assert_eq!(graph.node_property("a", "kind"), Some(PropertyValue::String("computation".into())));
        let all = graph.node_property("a", "__all").unwrap();
        assert!(matches!(all, PropertyValue::Object(ref map) if map.get("score") == Some(&PropertyValue::Number(1.0))));
        assert_eq!(graph.node_property("a", "score"), Some(PropertyValue::Number(1.0)));
        assert_eq!(graph.node_property("a", "nonexistent"), None);
        assert_eq!(graph.node_property("missing-node", "id"), None);
    });
}

#[test]
fn manifest_helpers_merge_graph_and_manifest_kinds() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        assert_eq!(graph.manifest().map(|m| m.id.as_str()), Some("flow-dag"));
        let node_kinds = manifest_node_kinds(&graph);
        assert!(node_kinds.iter().any(|k| k == "computation"));
        assert!(node_kinds.iter().any(|k| k == "select"), "manifest-only kind should be included");
        let edge_kinds = manifest_edge_kinds(&graph);
        assert!(edge_kinds.iter().any(|k| k == "wire"));
        let port_kinds = manifest_port_kinds(&graph);
        assert!(port_kinds.iter().any(|k| k == "in"));
        let props = manifest_property_names(&graph);
        for expected in ["id", "name", "kind", "label", "text", "score"] {
            assert!(props.iter().any(|p| p == expected), "missing property {expected}");
        }
    });
}

#[test]
fn subgraph_fixture_json_filters_to_requested_ids() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let node_ids = BTreeSet::from(["a".to_string(), "b".to_string()]);
        let edge_ids = BTreeSet::from(["e1".to_string()]);
        let json = graph.subgraph_fixture_json(&node_ids, &edge_ids).unwrap();
        let value: dsl_core::json::Value = dsl_core::json::parse(&json).unwrap();
        assert_eq!(value["nodes"].as_array().unwrap().len(), 2);
        assert_eq!(value["edges"].as_array().unwrap().len(), 1);
    });
}

#[test]
fn from_fixture_json_rejects_invalid_json() {
    block_on_test(async {
        let Err(err) = BoardQueryableGraph::from_fixture_json("not json", None) else { panic!("expected error") };
        assert!(matches!(err, GraphDslError::Json(_)));
    });
}

#[test]
fn from_puzzle3d_fixture_json_converts_objects_array() {
    block_on_test(async {
        let fixture = r#"{"objects": [{"id": "o1", "objectKind": "Cube", "name": "Box"}]}"#;
        let graph = BoardQueryableGraph::from_puzzle3d_fixture_json(fixture).unwrap();
        assert_eq!(graph.node_kind("o1").as_deref(), Some("Cube"));
        assert_eq!(graph.node_name("o1").as_deref(), Some("Box"));
        assert_eq!(graph.manifest().map(|m| m.id.as_str()), Some("puzzle3d-default"));
    });
}

#[test]
fn from_puzzle3d_fixture_json_passes_through_existing_nodes() {
    block_on_test(async {
        let fixture = r#"{"nodes": [{"id": "n1", "nodeKind": "Widget", "text": "N1"}]}"#;
        let graph = BoardQueryableGraph::from_puzzle3d_fixture_json(fixture).unwrap();
        assert_eq!(graph.node_kind("n1").as_deref(), Some("Widget"));
    });
}

#[test]
fn from_puzzle2d_and_puzzle5d_fixture_json_resolve_manifests() {
    block_on_test(async {
        let fixture = r#"{"nodes": [], "edges": []}"#;
        let g2 = BoardQueryableGraph::from_puzzle2d_fixture_json(fixture).unwrap();
        assert_eq!(g2.manifest().map(|m| m.id.as_str()), Some("puzzle2d-default"));
        let g5 = BoardQueryableGraph::from_puzzle5d_fixture_json(fixture).unwrap();
        assert_eq!(g5.manifest().map(|m| m.id.as_str()), Some("puzzle5d-default"));
    });
}
// #endregion 🔖️QueryableGraphTests

// #region 🔖️ErrorTests
#[test]
fn graph_dsl_error_display_messages() {
    assert_eq!(GraphDslError::UnterminatedString.to_string(), "unterminated string literal");
    assert_eq!(GraphDslError::UnexpectedChar('$').to_string(), "unexpected character '$'");
    assert_eq!(GraphDslError::EdgeTargetMissingPort.to_string(), "edge target requires @port");
    assert_eq!(GraphDslError::EmptyPattern.to_string(), "empty pattern");
    assert_eq!(GraphDslError::UnsupportedMutation.to_string(), "mutating jack clauses are not supported on this graph domain");
    assert_eq!(GraphDslError::UnknownProcedure("bogus".into()).to_string(), "unknown CALL procedure 'bogus'");
    assert_eq!(GraphDslError::ProcedureArity { name: "nodeKinds".into(), expected: 0, found: 1 }.to_string(), "procedure 'nodeKinds' expects 0 argument(s), got 1");
    let unexpected = GraphDslError::UnexpectedToken { expected: "ident".into(), found: "Eof".into() };
    assert_eq!(unexpected.to_string(), "expected ident, got Eof");
}

#[test]
fn parse_error_on_unexpected_char() {
    block_on_test(async {
        // `{`/`}` are valid tokens in `dsl_core`'s shared alphabet (map/object-literal braces)
        // but aren't part of Jack's own grammar (no map literals) — Jack rejects them itself,
        // hence `UnexpectedChar` rather than a `dsl_core`-surfaced `Lex` error.
        let err = parse("MATCH (a:x) { WHERE").unwrap_err();
        assert!(matches!(err, GraphDslError::UnexpectedChar('{')));
    });
}

#[test]
fn parse_error_on_char_outside_dsl_core_alphabet_reports_lex_error() {
    block_on_test(async {
        // `?` isn't lexable by `dsl_core` at all (unlike `{`/`}` above, which lex fine but aren't
        // valid Jack syntax) — `os_dsl::lex` itself fails, surfaced verbatim as `Lex`.
        let err = parse("MATCH (a:x) ? WHERE").unwrap_err();
        assert!(matches!(err, GraphDslError::Lex(_)));
        assert!(err.to_string().contains("unexpected character '?'"), "got: {err}");
    });
}

#[test]
fn parse_error_on_lone_bang_reports_lex_error() {
    block_on_test(async {
        // A stray `!` not followed by `=` isn't a token in Jack's grammar at all (`dsl_core` has
        // no relational operators, and Jack only special-cases `!=`).
        let err = parse("MATCH (a:x) WHERE a.p ! 1").unwrap_err();
        assert!(matches!(err, GraphDslError::Lex(_)));
    });
}

#[test]
fn parse_error_on_unterminated_string() {
    block_on_test(async {
        let err = parse("MATCH (a:x) WHERE a.name = 'oops").unwrap_err();
        assert!(matches!(err, GraphDslError::UnterminatedString));
    });
}
// #endregion 🔖️ErrorTests

// #region 🔖️LexerAndLanguageServiceTests
#[test]
fn tokenize_classifies_clause_and_operator_tokens() {
    block_on_test(async {
        let spans = tokenize("MATCH (a:x)--[:wire]->(b:y) WHERE a.p = 1 AND b.q != 'v' RETURN a.p");
        assert!(spans.iter().any(|s| s.class == TokenClass::Keyword));
        assert!(spans.iter().any(|s| s.class == TokenClass::Ident));
        assert!(spans.iter().any(|s| s.class == TokenClass::Number));
        assert!(spans.iter().any(|s| s.class == TokenClass::String));
        assert!(spans.iter().any(|s| s.class == TokenClass::Operator));
        assert!(spans.iter().any(|s| s.class == TokenClass::Punctuation));
    });
}

#[test]
fn tokenize_marks_unterminated_string_as_error_class() {
    block_on_test(async {
        let spans = tokenize("MATCH (a:x) WHERE a.p = 'unterminated");
        assert!(spans.iter().any(|s| s.class == TokenClass::Error));
    });
}

#[test]
fn tokenize_never_panics_on_stray_symbols() {
    block_on_test(async {
        // 🩹️ `#` is now a legitimate comment starter (unified with the rest of the DSL engine, so
        // it swallows the remainder of the line) — the stray-symbol probes moved off it.
        let spans = tokenize("MATCH (a:x) ~ ^ RETURN a");
        assert!(spans.iter().any(|s| s.class == TokenClass::Ident && s.end - s.start == 1));
    });
}

#[test]
fn tokenize_treats_hash_as_a_comment_to_end_of_line() {
    block_on_test(async {
        let source = "MATCH (a:x) # a trailing comment\nRETURN a";
        let comment_start = source.find('#').unwrap();
        let line_end = source.find('\n').unwrap();
        let spans = tokenize(source);
        assert!(!spans.iter().any(|s| s.start >= comment_start && s.start < line_end), "no token should start inside the comment body: {spans:?}");
        assert!(spans.iter().any(|s| s.class == TokenClass::Keyword));
    });
}

#[test]
fn format_query_is_idempotent_and_normalizes_whitespace() {
    block_on_test(async {
        let once = format("match(a:x)--[:wire]->(b:y) where a.p=1 and b.q!='v' return a.p,b.q").unwrap();
        assert!(once.contains("MATCH"));
        assert!(once.contains(" AND "));
        assert!(once.contains(" = "));
        let twice = format(&once).unwrap();
        assert_eq!(once, twice);
    });
}

#[test]
fn format_rejects_unterminated_string() {
    block_on_test(async {
        let err = format("MATCH (a:x) WHERE a.p = 'oops").unwrap_err();
        assert!(matches!(err, GraphDslError::UnterminatedString));
    });
}

#[test]
fn complete_after_colon_suggests_node_then_edge_kinds() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let node_source = "MATCH (a:c";
        let node_completions = complete(&graph, node_source, node_source.len());
        assert!(node_completions.iter().any(|c| c.label == "computation"));
        let edge_source = "MATCH (a:computation)--[:w";
        let edge_completions = complete(&graph, edge_source, edge_source.len());
        assert!(edge_completions.iter().any(|c| c.label == "wire"));
    });
}

#[test]
fn complete_after_at_suggests_port_kinds() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (a:computation@i";
        let completions = complete(&graph, source, source.len());
        assert!(completions.iter().any(|c| c.label == "in"));
    });
}

#[test]
fn complete_after_dot_suggests_property_names() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (a:computation) RETURN a.sc";
        let completions = complete(&graph, source, source.len());
        assert!(completions.iter().any(|c| c.label == "score"));
    });
}

#[test]
fn complete_suggests_bound_variable_when_prefix_does_not_match_logic_keywords() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (abc:computation) WHERE ab";
        let completions = complete(&graph, source, source.len());
        assert!(completions.iter().any(|c| c.label == "abc" && c.kind == "variable"));
    });
}

#[test]
fn complete_in_where_clause_suggests_logic_keywords() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (a:computation) WHERE a.score = 1 AN";
        let completions = complete(&graph, source, source.len());
        assert!(completions.iter().any(|c| c.label == "AND"));
    });
}

#[test]
fn complete_at_start_suggests_clause_keywords() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let completions = complete(&graph, "MA", 2);
        assert!(completions.iter().any(|c| c.label == "MATCH"));
    });
}

#[test]
fn hover_reports_keyword_and_bound_variable() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (a:computation) WHERE a.score = 1 RETURN a";
        let match_pos = source.find("MATCH").unwrap();
        assert!(hover(&graph, source, match_pos + 1).unwrap().contents.contains("keyword"));
        // 🩹️ `hover_word_at` folds `:`/`.` into the word span, so a bound variable only resolves
        // in isolation when nothing follows it — the trailing standalone `a` in `RETURN a`.
        let var_pos = source.rfind('a').unwrap();
        assert!(hover(&graph, source, var_pos + 1).unwrap().contents.contains("Bound variable"));
    });
}

#[test]
fn hover_matches_bare_node_kind_edge_kind_and_property_words() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let source = "computation wire score";
        assert!(hover(&graph, source, 3).unwrap().contents.contains("Node kind"));
        let edge_pos = source.find("wire").unwrap();
        assert!(hover(&graph, source, edge_pos + 1).unwrap().contents.contains("Edge kind"));
        let prop_pos = source.find("score").unwrap();
        assert!(hover(&graph, source, prop_pos + 1).unwrap().contents.contains("Property"));
    });
}

#[test]
fn hover_returns_none_for_whitespace() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        assert!(hover(&graph, "MATCH (a:x)   RETURN a", 12).is_none());
    });
}

#[test]
fn lint_flags_unknown_node_kind() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let diags = lint(&graph, "MATCH (a:nonexistentKind) RETURN a");
        assert!(diags.iter().any(|d| d.code.as_deref() == Some("jack/unknown-node-kind")));
    });
}

#[test]
fn lint_flags_unbound_variable() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let diags = lint(&graph, "MATCH (a:computation) RETURN b");
        assert!(diags.iter().any(|d| d.code.as_deref() == Some("jack/unbound-variable")));
    });
}

#[test]
fn lint_reports_parse_errors() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let diags = lint(&graph, "MATCH (a:computation");
        assert!(diags.iter().any(|d| d.code.as_deref() == Some("jack/parse-error")));
    });
}

#[test]
fn lint_clean_query_has_no_diagnostics() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let diags = lint(&graph, "MATCH (a:computation) RETURN a.name");
        assert!(diags.is_empty());
    });
}

#[test]
fn semantic_tokens_mirror_tokenize_classes() {
    block_on_test(async {
        let tokens = semantic_tokens("MATCH (a:x) RETURN a");
        assert!(tokens.iter().any(|t| t.class == "keyword"));
        assert!(tokens.iter().any(|t| t.class == "ident"));
    });
}
// #endregion 🔖️LexerAndLanguageServiceTests

// #region 🔖️ParserAndExecutorTests
#[test]
fn parse_delete_set_merge_clauses() {
    block_on_test(async {
        let q = parse("MATCH (a:x) DELETE a").unwrap();
        assert!(matches!(q.clauses[1], Clause::Delete(ref vars) if vars == &vec!["a".to_string()]));
        let q = parse("MATCH (a:x) SET a.name = 'v'").unwrap();
        assert!(matches!(q.clauses[1], Clause::Set(ref items) if items.len() == 1 && items[0].prop == "name"));
        let q = parse("MERGE (a:x)").unwrap();
        assert!(matches!(q.clauses[0], Clause::Merge(_)));
    });
}

#[test]
fn parse_where_and_or_precedence() {
    block_on_test(async {
        let q = parse("MATCH (a:x) WHERE a.p = 1 AND a.q = 2 OR a.r != 3").unwrap();
        let Clause::Where(expr) = &q.clauses[1] else { panic!("expected where") };
        assert!(matches!(expr, Expr::Or(_, _)));
    });
}

// #region 🔖️WithUnwindCallTests
#[test]
fn parse_with_clause() {
    block_on_test(async {
        let q = parse("MATCH (a:x) WITH a, a.name RETURN a").unwrap();
        let Clause::With(items) = &q.clauses[1] else { panic!("expected with") };
        assert_eq!(items.len(), 2);
        assert!(matches!(&items[0], ReturnItem::Var(v) if v == "a"));
        assert!(matches!(&items[1], ReturnItem::Property { var, prop } if var == "a" && prop == "name"));
    });
}

#[test]
fn parse_unwind_clause() {
    block_on_test(async {
        let q = parse("MATCH (a:x) UNWIND a.items AS item RETURN item").unwrap();
        let Clause::Unwind(clause) = &q.clauses[1] else { panic!("expected unwind") };
        assert!(matches!(&clause.source, ReturnItem::Property { var, prop } if var == "a" && prop == "items"));
        assert_eq!(clause.var, "item");
    });
}

#[test]
fn parse_call_clause_with_positional_args() {
    block_on_test(async {
        let q = parse("CALL myProc(1, \"two\", true)").unwrap();
        let Clause::Call(clause) = &q.clauses[0] else { panic!("expected call") };
        assert_eq!(clause.name, "myProc");
        assert_eq!(clause.args, vec![PropertyValue::Number(1.0), PropertyValue::String("two".to_string()), PropertyValue::Bool(true)]);
    });
}

#[test]
fn execute_with_projects_named_vars_and_drops_the_rest() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation)--[:wire]--(b:slider) WITH a RETURN a.name, b.name").unwrap();
        assert!(!result.rows.is_empty());
        for row in &result.rows {
            assert_eq!(row[0], PropertyValue::String("A".to_string()));
            assert_eq!(row[1], PropertyValue::Null, "b was projected out of scope by WITH a, so b.name must be null");
        }
    });
}

#[test]
fn execute_with_where_filters_the_projected_bindings() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:slider) WITH a WHERE a.name = 'B' RETURN a.name").unwrap();
        assert_eq!(result.rows, vec![vec![PropertyValue::String("B".to_string())]]);
    });
}

#[test]
fn execute_with_property_item_keeps_its_source_var_resolvable() {
    block_on_test(async {
        // 🔭️ `ReturnItem` carries no alias, so `WITH a.name` keeps the whole `a` entity in
        // scope (there is no other way for a later `a.name` to still resolve) — see
        // `project_binding`'s doc comment for the reasoning.
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation) WITH a.name RETURN a.name").unwrap();
        assert_eq!(result.rows, vec![vec![PropertyValue::String("A".to_string())]]);
    });
}

#[test]
fn execute_unwind_over_a_property_expression_producing_a_list() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(list_property_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation) UNWIND a.tags AS tag RETURN tag").unwrap();
        assert_eq!(result.rows, vec![vec![PropertyValue::String("x".to_string())], vec![PropertyValue::String("y".to_string())], vec![PropertyValue::String("z".to_string())],]);
    });
}

#[test]
fn execute_unwind_over_an_already_bound_list_value() {
    block_on_test(async {
        // 🌀️ Chained UNWIND: the outer unwind's per-row `row` binding lives in `values` (not
        // a graph property), so the inner `UNWIND row AS cell` exercises the `Var`-sourced
        // (rather than `Property`-sourced) list-expression path.
        let graph = BoardQueryableGraph::from_fixture_json(list_property_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation) UNWIND a.matrix AS row UNWIND row AS cell RETURN cell").unwrap();
        assert_eq!(result.rows, vec![vec![PropertyValue::Number(1.0)], vec![PropertyValue::Number(2.0)], vec![PropertyValue::Number(3.0)], vec![PropertyValue::Number(4.0)]]);
    });
}

#[test]
fn execute_unwind_of_an_empty_list_yields_zero_rows() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(list_property_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation) UNWIND a.empty AS e RETURN e").unwrap();
        assert!(result.rows.is_empty());
    });
}

#[test]
fn execute_call_known_procedure_yields_its_registered_column() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(list_property_fixture(), None).unwrap();
        let result = run_query(&graph, "CALL nodeKinds() RETURN kind").unwrap();
        assert_eq!(result.rows, vec![vec![PropertyValue::String("computation".to_string())], vec![PropertyValue::String("slider".to_string())]]);
    });
}

#[test]
fn execute_call_unknown_procedure_reports_a_precise_error() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(list_property_fixture(), None).unwrap();
        let err = run_query(&graph, "CALL bogus()").unwrap_err();
        assert!(matches!(err, GraphDslError::UnknownProcedure(ref name) if name == "bogus"), "got {err:?}");
    });
}

#[test]
fn execute_call_procedure_arity_mismatch_reports_a_precise_error() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(list_property_fixture(), None).unwrap();
        let err = run_query(&graph, "CALL nodeKinds(1)").unwrap_err();
        assert!(matches!(err, GraphDslError::ProcedureArity { ref name, expected: 0, found: 1 } if name == "nodeKinds"), "got {err:?}");
    });
}
// #endregion 🔖️WithUnwindCallTests

#[test]
fn lexer_accepts_both_single_and_double_quoted_strings_and_always_prints_double_quoted() {
    block_on_test(async {
        let single = parse("MATCH (a:x) WHERE a.name = 'alpha' RETURN a").unwrap();
        let double = parse("MATCH (a:x) WHERE a.name = \"alpha\" RETURN a").unwrap();
        assert_eq!(single, double, "single- and double-quoted string literals must parse identically");
        let printed = format("MATCH (a:x) WHERE a.name = 'alpha' RETURN a").unwrap();
        assert!(printed.contains("\"alpha\""), "must always print double-quoted: {printed}");
        assert!(!printed.contains('\''), "must never print single-quoted: {printed}");
    });
}

#[test]
fn parse_unexpected_token_error_has_expected_and_found() {
    block_on_test(async {
        let err = parse("MATCH a:x)").unwrap_err();
        let GraphDslError::UnexpectedToken { expected, found } = err else { panic!("expected UnexpectedToken") };
        assert_eq!(expected, "LParen");
        assert!(found.contains("Ident"));
    });
}

#[test]
fn execute_where_clause_filters_bindings() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:slider) WHERE a.name = 'B' RETURN a.name").unwrap();
        assert_eq!(result.rows, vec![vec![PropertyValue::String("B".into())]]);
    });
}

#[test]
fn execute_and_or_expressions() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let and_result = run_query(&graph, "MATCH (a:slider) WHERE a.name = 'B' AND a.kind = 'slider' RETURN a.name").unwrap();
        assert_eq!(and_result.rows.len(), 1);
        let or_result = run_query(&graph, "MATCH (a:slider) WHERE a.name = 'B' OR a.name = 'C' RETURN a.name").unwrap();
        assert_eq!(or_result.rows.len(), 2);
    });
}

#[test]
fn execute_rejects_mutating_clauses() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        for query in ["CREATE (a:x)", "MATCH (a:x) DELETE a", "MATCH (a:x) SET a.p = 1", "MERGE (a:x)"] {
            let err = run_query(&graph, query).unwrap_err();
            assert!(matches!(err, GraphDslError::UnsupportedMutation), "query {query} should reject mutation");
        }
    });
}

#[test]
fn execute_undirected_edge_matches_both_directions() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let forward = run_query(&graph, "MATCH (a:computation)--[:wire]--(b:slider) RETURN a.name, b.name").unwrap();
        let reverse = run_query(&graph, "MATCH (b:slider)--[:wire]--(a:computation) RETURN a.name, b.name").unwrap();
        assert!(!forward.rows.is_empty());
        assert!(!reverse.rows.is_empty());
    });
}

#[test]
fn execute_multiple_match_patterns_join_bindings() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation), (b:slider) RETURN a.name, b.name").unwrap();
        assert_eq!(result.rows.len(), 2);
    });
}

#[test]
fn execute_returns_graph_kind_when_returning_bound_entities() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation)--[e:wire]--(b:slider) RETURN a, e, b").unwrap();
        assert_eq!(result.kind, QueryResultKind::Graph);
        assert!(result.graph_fixture_json.is_some());
    });
}

#[test]
fn execute_returns_table_kind_for_property_projection() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation) RETURN a.name").unwrap();
        assert_eq!(result.kind, QueryResultKind::Table);
    });
}

#[test]
fn execute_with_no_return_clause_yields_empty_table() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation)").unwrap();
        assert!(result.columns.is_empty());
        assert!(result.rows.is_empty());
    });
}

#[test]
fn run_query_json_serializes_result() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let json = run_query_json(&graph, "MATCH (a:computation) RETURN a.name").unwrap();
        let value: dsl_core::json::Value = dsl_core::json::parse(&json).unwrap();
        assert_eq!(value["columns"][0], "a.name");
    });
}

#[test]
fn empty_pattern_error_is_reachable_via_pattern_construction() {
    block_on_test(async {
        let pattern = Pattern { nodes: vec![], edge: None };
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let err = match_patterns(&graph, std::slice::from_ref(&pattern)).unwrap_err();
        assert!(matches!(err, GraphDslError::EmptyPattern));
    });
}
// #endregion 🔖️ParserAndExecutorTests

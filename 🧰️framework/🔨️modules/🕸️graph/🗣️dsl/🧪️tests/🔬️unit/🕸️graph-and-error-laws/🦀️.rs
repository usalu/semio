/// 🕸️ Graph and error laws of the graph DSL: the split endpoint fixtures, the queryable graph, snapshot and manifest helpers and the parse and lex error reports.

// #region 🔖️Fixtures
/// 🧵️ Small hand-built graph exercising every `split_endpoint` branch: exact handle match,
/// mapped/unmapped `@` and `:` splits, and the plain-id fallback.
fn split_endpoint_fixture() -> &'static str {
    r#"{
  "manifestId": "future.neutral.geometry",
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
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
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
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
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
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
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
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
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
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
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
        let manifest = <semio_framework_graph_manifest::Manifest as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"schema":"manifest","id":"future.neutral.geometry","nodeKinds":[{"id":"Declared"}]}"#,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), Some(manifest)).unwrap();
        assert_eq!(graph.manifest().map(|m| m.id.as_str()), Some("future.neutral.geometry"));
        let node_kinds = manifest_node_kinds(&graph);
        assert!(node_kinds.iter().any(|k| k == "computation"));
        assert!(node_kinds.iter().any(|k| k == "Declared"), "manifest-only kind should be included");
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
fn subgraph_snapshot_json_filters_to_requested_ids() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let node_ids = BTreeSet::from(["a".to_string(), "b".to_string()]);
        let edge_ids = BTreeSet::from(["e1".to_string()]);
        let json = graph.subgraph_snapshot_json(&node_ids, &edge_ids).unwrap();
        let value: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&json,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(value["nodes"].as_array().unwrap().len(), 2);
        assert_eq!(value["edges"].as_array().unwrap().len(), 1);
    });
}

#[test]
fn from_host_snapshot_json_rejects_invalid_json() {
    block_on_test(async {
        let Err(err) = BoardQueryableGraph::from_host_snapshot_json("not json", None) else { panic!("expected error") };
        assert!(matches!(err, GraphDslError::Json(_)));
    });
}

#[test]
fn from_object_snapshot_json_converts_objects_array() {
    block_on_test(async {
        let fixture = r#"{"objects": [{"id": "o1", "objectKind": "Cube", "name": "Box"}]}"#;
        let graph = BoardQueryableGraph::from_object_snapshot_json(fixture, None).unwrap();
        assert_eq!(graph.node_kind("o1").as_deref(), Some("Cube"));
        assert_eq!(graph.node_name("o1").as_deref(), Some("Box"));
        assert_eq!(graph.manifest().map(|m| m.id.as_str()), None);
    });
}

#[test]
fn from_object_snapshot_json_passes_through_existing_nodes() {
    block_on_test(async {
        let fixture = r#"{"nodes": [{"id": "n1", "nodeKind": "Widget", "text": "N1"}]}"#;
        let graph = BoardQueryableGraph::from_object_snapshot_json(fixture, None).unwrap();
        assert_eq!(graph.node_kind("n1").as_deref(), Some("Widget"));
    });
}

#[test]
fn explicit_manifest_consumption_follows_portable_owner_corpus() {
    let corpus = semio_framework_pack_json::parse(include_str!("../../../../🛂️manifest/🧪️tests/🧫️fixtures/🧩️consumption/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for row in corpus["cases"].as_array().unwrap() {
        let manifest = if row["manifest"].is_null() { None } else { Some(<semio_framework_graph_manifest::Manifest as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&row["manifest"])).unwrap()) };
        let actual = BoardQueryableGraph::from_host_snapshot_json(&semio_framework_pack_json::to_string(&row["snapshot"]), manifest);
        assert_eq!(actual.is_ok(), row["expected"]["accepted"].as_bool().unwrap(), "{}", row["id"]);
        if let Ok(graph) = actual {
            assert_eq!(graph.manifest().map(|value| value.id.as_str()), row["expected"]["manifestId"].as_str());
            let expected: Vec<String> = row["expected"]["nodeKinds"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_string()).collect();
            assert_eq!(manifest_node_kinds(&graph), expected, "{}", row["id"]);
        }
    }
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

use super::*;

#[semio_framework_async_macros::async_test]
async fn dump_example_dsl_when_requested() {
    if std::env::var("DUMP_DAG_EXAMPLE").is_ok() {
        use crate::snapshot::schema::DagSnapshot;
        use crate::{dag_content_child_with_owner, DagFixtureEdge, DagNodeSpec, DAG_DOCUMENT_SCHEMA};
        let nodes = vec![DagNodeSpec { id: "slider-a".into(), name: "A".into(), ..Default::default() }, DagNodeSpec { id: "slider-b".into(), name: "B".into(), x: 200.0, ..Default::default() }];
        let edges = vec![DagFixtureEdge { id: "edge-1".into(), source: "slider-a@out".into(), target: "slider-b@in".into(), ..Default::default() }];
        let content = dag_content_child_with_owner(nodes, edges);
        let snapshot = DagSnapshot { schema: DAG_DOCUMENT_SCHEMA.into(), content };
        println!("{}", print_dsl(&snapshot));
    }
}

#[test]
fn demo_graph_matches_the_language_neutral_json_oracle() {
    let snapshot = parse_dsl(DAG_EXAMPLE_TEXT).expect("demo DSL");
    let graph = semio_framework_artifact_infinite_dag::DagSnapshot::from(&snapshot);
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../../../../📚️examples/🎬️demo/🧫️fixtures/🧾️scene.json")).expect("demo JSON oracle");
    let observed = serde_json::json!({
        "nodes": graph.nodes.iter().map(|node| (&node.id, &node.name, node.x, node.y)).collect::<Vec<_>>(),
        "edges": graph.edges.iter().map(|edge| (&edge.id, &edge.source, &edge.target)).collect::<Vec<_>>(),
    });
    assert_eq!(observed, expected);
    let reparsed = <semio_framework_artifact_infinite_dag::DagSnapshot as store::ArtifactDsl>::parse_dsl(&print_dsl(&snapshot)).expect("shared graph grammar");
    assert_eq!(reparsed, graph);
}

#[semio_framework_async_macros::async_test]
async fn example_fixture_dsl_round_trips() {
    let document = parse_dsl(DAG_EXAMPLE_TEXT).expect("parse default fixture");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn fused_edge_arrow_wire_parses_labeled_endpoints() {
    let parsed = dsl::parse_wire_text("a -e1:Connection> b:Node@out").expect("parse fused edge");
    assert_eq!(parsed.edge_label.id.as_deref(), Some("e1"));
    assert_eq!(parsed.edge_label.kind.as_deref(), Some("Connection"));
    assert_eq!(parsed.from.id, "a");
    assert!(parsed.edge.as_ref().map(|(d, _)| *d).unwrap_or(false));
}

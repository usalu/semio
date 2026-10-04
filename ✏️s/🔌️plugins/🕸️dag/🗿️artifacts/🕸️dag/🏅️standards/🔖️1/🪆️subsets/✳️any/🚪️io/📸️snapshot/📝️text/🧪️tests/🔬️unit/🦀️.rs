use super::*;

#[semio_framework_async_macros::async_test]
async fn dump_example_dsl_when_requested() {
    if std::env::var("DUMP_DAG_EXAMPLE").is_ok() {
        use crate::snapshot::schema::DagSnapshot;
        use crate::{DagHostSnapshotEdge, DagNodeSpec, DagScene, DAG_DOCUMENT_SCHEMA};
        let nodes = vec![DagNodeSpec { id: "slider-a".into(), name: "A".into(), ..Default::default() }, DagNodeSpec { id: "slider-b".into(), name: "B".into(), x: 200.0, ..Default::default() }];
        let edges = vec![DagHostSnapshotEdge { id: "edge-1".into(), source: "slider-a@out".into(), target: "slider-b@in".into(), ..Default::default() }];
        let snapshot = DagSnapshot { schema: DAG_DOCUMENT_SCHEMA.into(), content: crate::dag_content_child_handle(&DagScene { nodes, edges }) };
        println!("{}", print_dsl(&snapshot));
    }
}

#[test]
fn demo_graph_matches_the_language_neutral_json_oracle() {
    let snapshot = crate::examples::demo::snapshot();
    let graph = crate::examples::demo::scene();
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../../../../📚️examples/🎬️demo/🧫️fixtures/🧾️scene.json")).expect("demo JSON oracle");
    let observed = serde_json::json!({
        "nodes": graph.nodes.iter().map(|node| (&node.id, &node.name, node.x, node.y)).collect::<Vec<_>>(),
        "edges": graph.edges.iter().map(|edge| (&edge.id, &edge.source, &edge.target)).collect::<Vec<_>>(),
    });
    assert_eq!(observed, expected);
    let reparsed = parse_dsl(&print_dsl(&snapshot)).expect("literal parent grammar");
    assert_eq!(reparsed.schema,snapshot.schema);
    assert_eq!(reparsed.content.child_id,snapshot.content.child_id);
    assert_eq!(reparsed.content.target,snapshot.content.target);
    assert_eq!(crate::dag_derivable_scene(&reparsed), Some(graph), "the literal parent names the derivable demo content");
}

#[semio_framework_async_macros::async_test]
async fn example_fixture_dsl_round_trips() {
    let document = parse_dsl(crate::examples::demo::PRIMARY_TEXT).expect("parse default fixture");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn fused_edge_arrow_wire_parses_labeled_endpoints() {
    let parsed = semio_framework_dsl_record::parse_wire_text("a -e1:Connection> b:Node@out").expect("parse fused edge");
    assert_eq!(parsed.edge_label.id.as_deref(), Some("e1"));
    assert_eq!(parsed.edge_label.kind.as_deref(), Some("Connection"));
    assert_eq!(parsed.from.id, "a");
    assert!(parsed.edge.as_ref().map(|(d, _)| *d).unwrap_or(false));
}

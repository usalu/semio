
use super::*;

trait DagChildOwnerOracle {
    fn expected() -> serde_json::Value;
}

struct SerdeJsonDagChildOwnerOracle;

impl DagChildOwnerOracle for SerdeJsonDagChildOwnerOracle {
    fn expected() -> serde_json::Value {
        serde_json::from_str(include_str!("../../🧫️fixtures/🧫️child-owner-isolation/🔣️.json")).expect("language-neutral DAG child-owner fixture")
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_kind_declares_the_graph_dag_component_kind() {
    assert_eq!(artifact_kind().id, "graph.dag");
    assert_eq!(artifact_kind().schema, DAG_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_matches_document_schema() {
    assert_eq!(default_snapshot().schema, DAG_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn node_edge_content_round_trips_through_the_composed_child_snapshot() {
    let document = default_snapshot();
    let scene = dag_working_scene(&document);
    let content = dag_content_snapshot_from_working(&scene.nodes, &scene.edges);
    let (nodes, edges) = working_from_dag_content_snapshot(&content);
    assert_eq!(nodes, scene.nodes);
    assert_eq!(edges, scene.edges);
}

#[semio_framework_async_macros::async_test]
async fn dag_working_scene_is_owned_by_the_exact_snapshot_child() {
    let owned = dag_content_child_with_owner(Vec::new(), Vec::new());
    let wire = dsl::json::to_json_string(&owned);
    let reconstructed: DagContentChild = dsl::json::from_json_str(&wire).expect("DAG child wire roundtrip");
    let observed = serde_json::json!({
        "ownedHasScene": owned.local_owner::<DagWorkingScene>().is_some(),
        "wireIdentityMatches": owned == reconstructed,
        "wireHasScene": reconstructed.local_owner::<DagWorkingScene>().is_some(),
    });

    assert_eq!(observed, SerdeJsonDagChildOwnerOracle::expected());
}

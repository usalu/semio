use super::*;

#[semio_framework_async_macros::async_test]
async fn artifact_kind_declares_the_graph_dag_component_kind() {
    assert_eq!(artifact_kind().id, "graph.dag");
    assert_eq!(artifact_kind().schema, DAG_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_matches_artifact_schema() {
    assert_eq!(default_snapshot().schema, DAG_DOCUMENT_SCHEMA);
}

/// 🌉️ The demo scene round-trips through the composed graph content exactly (native slots + typed properties).
#[semio_framework_async_macros::async_test]
async fn node_edge_content_round_trips_through_the_composed_child_snapshot() {
    let scene = examples::demo::scene();
    assert_eq!(dag_scene_of_content(&dag_content_snapshot(&scene)), scene);
}

/// 🌱️ Only the derivable contents (bundled demo, empty graph) have a genesis pack; any other child id is not derivable,
/// so a decoded parent never needs a working scene on its handle (design §20.15).
#[semio_framework_async_macros::async_test]
async fn only_derivable_children_have_a_genesis_pack() {
    use store::ArtifactPack;
    let demo = default_snapshot();
    let pack = genesis_dag_child_pack(&demo, "content", &demo.content.child_id).expect("the demo content is derivable");
    assert_eq!(dag_scene_of_content(&SemioGraphSnapshot::decode_pack(&pack).expect("genesis pack decodes")), examples::demo::scene());
    let empty = empty_snapshot();
    assert!(genesis_dag_child_pack(&empty, "content", &empty.content.child_id).is_some(), "the empty graph is derivable");
    let other = DagSnapshot { content: dag_content_child_handle(&DagScene { nodes: vec![schema::default_node_for_kind("note", "x", 0.0, 0.0)], edges: Vec::new() }), ..default_snapshot() };
    assert!(genesis_dag_child_pack(&other, "content", &other.content.child_id).is_none());
    assert!(genesis_dag_child_pack(&demo, "other-slot", &demo.content.child_id).is_none());
}

/// 🧬️ The projection names exactly the snapshot's declared child slots — what the live envelope load checks
/// before a decoded document may replace the store.
#[test]
fn the_child_restore_projection_names_every_declared_child_slot() {
    let snapshot = crate::default_snapshot();
    let projection = crate::dag_child_restore_projection(&snapshot).expect("the loaded-parent child projection");
    assert_eq!(projection.len(), <crate::DagSnapshot as semio_framework_schema_composition::ArtifactCompositionFields>::child_slots().len());
}

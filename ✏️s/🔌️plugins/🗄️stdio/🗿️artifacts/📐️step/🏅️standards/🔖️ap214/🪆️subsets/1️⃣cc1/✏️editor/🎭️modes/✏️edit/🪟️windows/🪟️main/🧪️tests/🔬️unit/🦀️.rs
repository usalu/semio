use super::*;
use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_editable_mesh_window_kit() {
    let def = definition();
    assert_eq!(def.id, MeshWindowKit::KIND_ID);
    assert!(def.actions.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = StepSnapshot::default();
    let _node = render(&document);
}

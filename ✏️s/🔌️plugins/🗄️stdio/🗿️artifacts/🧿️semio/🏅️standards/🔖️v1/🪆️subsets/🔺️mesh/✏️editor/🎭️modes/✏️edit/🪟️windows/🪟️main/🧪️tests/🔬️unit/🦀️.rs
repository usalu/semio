use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_editable_mesh_window_kit() {
    let def = definition();
    assert_eq!(def.id, MeshWindowKit::KIND_ID);
    let [action] = def.actions.as_slice() else { panic!("the edit window carries exactly the set-vertex action") };
    assert_eq!(action.id, "set-vertex");
    assert_eq!(action.semantics.execution.interactive_job, semio_framework_plugin::InteractiveJobClassification::Migrated);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = SemioMeshSnapshot::default();
    let _node = render(&document);
}

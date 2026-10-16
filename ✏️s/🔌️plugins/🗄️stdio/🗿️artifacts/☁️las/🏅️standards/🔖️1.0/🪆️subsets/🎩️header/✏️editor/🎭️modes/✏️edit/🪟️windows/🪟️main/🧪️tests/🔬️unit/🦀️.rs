use crate::standards::v1_0::subsets::any::schema::snapshot::LasSnapshot;
use semio_framework_plugin::MeshWindowKit;
use crate::editor::las::modes::edit::windows::main::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_editable_mesh_window_kit() {
    let def = definition();
    assert_eq!(def.id, MeshWindowKit::KIND_ID);
    assert!(def.actions.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = LasSnapshot::default();
    let _node = render(&document);
}

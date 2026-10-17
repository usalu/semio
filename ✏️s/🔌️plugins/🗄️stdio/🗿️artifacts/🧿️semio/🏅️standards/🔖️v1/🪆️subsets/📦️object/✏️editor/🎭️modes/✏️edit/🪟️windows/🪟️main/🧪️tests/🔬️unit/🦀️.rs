use crate::schema::snapshot::SemioObjectSnapshot;
use semio_framework_plugin::MeshWindowKit;
#[allow(unused_imports)]
use semio_framework_plugin::{world3d_camera_json, world3d_meshes_json_from_kinds, world3d_selection_json, BuiltNode, MeshView, WindowKindDefinition, WindowKit};
use crate::editor::modes::edit::windows::main::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_editable_mesh_window_kit() {
    let def = definition();
    assert_eq!(def.id, MeshWindowKit::KIND_ID);
    assert!(def.actions.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = SemioObjectSnapshot::default();
    let _node = render(&document);
}

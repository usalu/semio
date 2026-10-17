use crate::standards::v_r12::subsets::any::schema::snapshot::DxfSnapshot;
use semio_framework_plugin::MeshWindowKit;
#[allow(unused_imports)]
use semio_framework_plugin::{mesh_from_kind, world3d_camera_json, world3d_selection_json, BuiltNode, MeshView, WindowKindDefinition, WindowKit};
use crate::editor::dxf::modes::edit::windows::main::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_editable_mesh_window_kit() {
    let def = definition();
    assert_eq!(def.id, MeshWindowKit::KIND_ID);
    assert!(def.actions.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = DxfSnapshot::default();
    let _node = render(&document);
}

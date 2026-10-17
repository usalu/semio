use crate::schema::snapshot::SemioKitSnapshot;
use semio_framework_plugin::MeshWindowKit;
#[allow(unused_imports)]
use semio_framework_plugin::{world3d_camera_json, world3d_meshes_json_from_kinds, world3d_selection_json, BuiltNode, MeshView, WindowKindDefinition, WindowKit};
use crate::viewer::modes::view::windows::main::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_shared_mesh_window_kit() {
    assert_eq!(definition().id, MeshWindowKit::KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = SemioKitSnapshot::default();
    let _node = render(&document);
}

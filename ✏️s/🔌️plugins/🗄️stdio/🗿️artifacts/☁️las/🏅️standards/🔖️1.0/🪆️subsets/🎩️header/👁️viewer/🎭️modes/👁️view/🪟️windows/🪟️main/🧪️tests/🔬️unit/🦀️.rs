use crate::standards::v1_0::subsets::any::schema::snapshot::LasSnapshot;
use semio_framework_plugin::MeshWindowKit;
use crate::viewer::las::modes::view::windows::main::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_shared_mesh_window_kit() {
    assert_eq!(definition().id, MeshWindowKit::KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = LasSnapshot::default();
    let _node = render(&document);
}

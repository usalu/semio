use crate::standards::v_ascii::subsets::any::schema::snapshot::StlSnapshot;
use semio_framework_plugin::MeshWindowKit;
use crate::viewer::stl::modes::view::windows::main::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_shared_mesh_window_kit() {
    assert_eq!(definition().id, MeshWindowKit::KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = StlSnapshot::default();
    let _node = render(&document);
}

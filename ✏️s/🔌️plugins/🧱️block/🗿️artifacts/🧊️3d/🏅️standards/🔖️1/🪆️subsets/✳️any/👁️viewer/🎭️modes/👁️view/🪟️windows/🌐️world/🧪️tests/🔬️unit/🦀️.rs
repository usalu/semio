
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_shared_mesh_window_kind() {
    let def = definition();
    assert_eq!(def.id, "framework.window.mesh");
    assert_eq!(def.body_key, "framework.window.mesh");
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_empty_document() {
    let document = crate::standards::v1::subsets::any::schema::empty_block3d_snapshot();
    let node = render(&document).expect("the empty document must still assemble a scene surface");
    assert!(matches!(node.component, semio_framework_plugin::Component::Surface(_)));
}

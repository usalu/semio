
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_frozen_mesh_window_kind() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = crate::standards::v1::subsets::any::schema::empty_block5d_snapshot();
    let _node = render(&document);
}

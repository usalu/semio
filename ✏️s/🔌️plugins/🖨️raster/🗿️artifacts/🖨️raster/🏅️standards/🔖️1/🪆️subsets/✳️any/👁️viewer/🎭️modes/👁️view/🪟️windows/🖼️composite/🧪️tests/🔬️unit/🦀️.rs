
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_frozen_image_window_kind() {
    let def = definition();
    assert_eq!(def.id, ImageWindowKit::KIND_ID);
    assert_eq!(def.body_key, ImageWindowKit::KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = crate::standards::v1::subsets::any::schema::empty_raster_document();
    let _node = render(&document).expect("bounded fixture");
}

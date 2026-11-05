use super::*;
use semio_plugin::app::ImageWindowKit;
use semio_plugin::WindowKit;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_image_window_kit() {
    let def = definition();
    assert_eq!(def.id, ImageWindowKit::KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = DwgSnapshot::default();
    assert!(render(&document).is_ok());
}

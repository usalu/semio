
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_canvas2d_navigator_window() {
    let def = definition();
    assert_eq!(def.id, RASTER_VIEW_WINDOW_NAVIGATOR);
    assert_eq!(def.body_key, RASTER_VIEW_BODY_NAVIGATOR);
    assert_eq!(def.surface_kind, SurfaceKind::Canvas2d);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = crate::schema::empty_raster_document();
    let _node = render(&document).expect("bounded fixture");
}

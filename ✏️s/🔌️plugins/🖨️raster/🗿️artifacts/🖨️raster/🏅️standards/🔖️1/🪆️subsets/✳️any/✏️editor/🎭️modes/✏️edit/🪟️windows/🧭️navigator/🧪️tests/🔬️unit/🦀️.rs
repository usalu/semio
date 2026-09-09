use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_paint2d_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, RASTER_PLAY_BODY_NAVIGATOR);
    assert!(matches!(definition.surface_kind, SurfaceKind::Paint2d));
}

use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_paint2d_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, RASTER_PLAY_BODY_COMPOSITE);
    assert!(matches!(definition.surface_kind, SurfaceKind::Paint2d));
    assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
}

#[semio_framework_async_macros::async_test]
async fn window_measures_surface_brush_and_eraser_groups() {
    let config = RasterConfig::default();
    let measures = window_measures(&config);
    assert_eq!(measures.len(), 2);
    assert!(measures.iter().any(|m| matches!(m, WindowMeasure::Group { id, .. } if id == "raster-utility-options-paintBrush")));
    assert!(measures.iter().any(|m| matches!(m, WindowMeasure::Group { id, .. } if id == "raster-utility-options-paintEraser")));
}

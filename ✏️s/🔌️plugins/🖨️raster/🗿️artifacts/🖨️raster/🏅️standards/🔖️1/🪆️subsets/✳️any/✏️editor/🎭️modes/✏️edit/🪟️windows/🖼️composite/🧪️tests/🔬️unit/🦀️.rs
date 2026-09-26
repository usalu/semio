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
    let measures = window_measures(&config, &crate::editor::raster::terminology::RasterPlayLabels::NATIVE_EN);
    assert_eq!(measures.len(), 2);
    assert!(measures.iter().any(|m| matches!(m, WindowMeasure::Group { id, .. } if id == "raster-utility-options-paintBrush")));
    assert!(measures.iter().any(|m| matches!(m, WindowMeasure::Group { id, .. } if id == "raster-utility-options-paintEraser")));
}

#[semio_framework_async_macros::async_test]
async fn brush_controls_share_color_hardness_and_resolved_german_labels() {
    let config = RasterConfig { brush_color: "#123456".into(), brush_hardness: 0.25, ..RasterConfig::default() };
    let measures = window_measures(&config, &crate::editor::raster::terminology::RasterPlayLabels::NATIVE_DE);
    let WindowMeasure::Group { label, children, .. } = &measures[0] else { panic!("brush group") };
    assert_eq!(label, "Pinsel");
    assert!(children.iter().any(|m| matches!(m, WindowMeasure::Slider { label, value, .. } if label.as_deref() == Some("Härte") && *value == 0.25)));
    assert!(children.iter().any(|m| matches!(m, WindowMeasure::Select { value, items, .. } if value == "#123456" && items.iter().any(|item| item.value == *value))));
}

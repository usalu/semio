
use super::*;

#[semio_framework_async_macros::async_test]
async fn raster_config_operation_round_trips_and_backwards_restores_snapshot() {
    let base = RasterConfig { brush_size: 24.0, ..Default::default() };
    let operation = RasterConfigMutation::SetBrushSize { value: 40.0 };
    let forward = operation.diff(&base).diff().clone();
    assert_eq!(forward.brush_size, 40.0);
    let backwards = operation.inverse(&base);
    assert_eq!(backwards, vec![RasterConfigMutation::Snapshot { config: base.clone() }]);
    assert_eq!(backwards[0].diff(&forward).diff().clone(), base);
}

#[semio_framework_async_macros::async_test]
async fn raster_config_operation_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::Snapshot { config: RasterConfig::default() });
    store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetBrushSize { value: 40.0 });
    store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetBrushOpacity { value: 0.5 });
    store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetCompositeViewport { viewport: Some(RasterConfigViewportSize { width: 640.0, height: 480.0 }) });
    store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetCompositeViewport { viewport: None });
    store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetCamera { camera: RasterCamera { x: 1.0, y: -2.0, zoom: 3.0 } });
    store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetActiveUtility { utility_id: "paintBrush".into() });
    store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetLocale { value: "de-DE".into() });
}

#[semio_framework_async_macros::async_test]
async fn raster_config_default_matches_ui_selectmarquee_utility() {
    let config = RasterConfig::default();
    assert_eq!(config.active_utility_id, "selectMarquee");
    assert_eq!(config.locale, "en-US");
    assert_eq!(config.brush_size, 24.0);
    assert_eq!(config.brush_opacity, 1.0);
}

#[semio_framework_async_macros::async_test]
async fn raster_config_dsl_round_trips() {
    let config = RasterConfig {
        brush_size: 40.0,
        brush_opacity: 0.5,
        composite_viewport: Some(RasterConfigViewportSize { width: 640.0, height: 480.0 }),
        camera: RasterCamera { x: 5.0, y: -3.0, zoom: 2.0 },
        active_utility_id: "paintBrush".into(),
        locale: "de-DE".into(),
    };
    store::os_store::test_support::assert_dsl_round_trip(&config);
}

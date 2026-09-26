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
}

#[semio_framework_async_macros::async_test]
async fn raster_config_default_matches_brush_controls() {
    let config = RasterConfig::default();
    assert_eq!(config.brush_size, 24.0);
    assert_eq!(config.brush_opacity, 1.0);
}

#[semio_framework_async_macros::async_test]
async fn raster_config_dsl_round_trips() {
    let config = RasterConfig { brush_size: 40.0, brush_opacity: 0.5, brush_color: "#e07020".into(), brush_hardness: 0.25, composite_viewport: Some(RasterConfigViewportSize { width: 640.0, height: 480.0 }), camera: RasterCamera { x: 5.0, y: -3.0, zoom: 2.0 } };
    store::os_store::test_support::assert_dsl_round_trip(&config);
}

#[semio_framework_async_macros::async_test]
async fn brush_style_shared_vectors_preserve_session_state_and_reject_invalid_values() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🖌️brush-style/🔣️.json")).unwrap();
    let base = RasterConfig::default();
    for row in fixture["cases"].as_array().unwrap() {
        let color = row["color"].as_str().unwrap().to_string();
        let hardness = row["hardness"].as_f64().unwrap();
        let color_op = RasterConfigMutation::SetBrushColor { value: color.clone() };
        let next = color_op.diff(&base).diff().clone();
        let next = RasterConfigMutation::SetBrushHardness { value: hardness }.diff(&next).diff().clone();
        assert_eq!(next.brush_color, color);
        assert_eq!(next.brush_hardness, hardness);
        assert_eq!(next.brush_rgba().to_vec(), row["rgba"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u8).collect::<Vec<_>>());
        let mut oracle: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&base)).unwrap();
        oracle["brushColor"] = row["color"].clone();
        oracle["brushHardness"] = row["hardness"].clone();
        let expected: RasterConfig = dsl::json::from_json_str(&oracle.to_string()).unwrap();
        assert_eq!(next, expected);
        assert_eq!(color_op.inverse(&base)[0].diff(&next).diff(), &base);
        store::os_store::test_support::assert_op_line_round_trip(&color_op);
        store::os_store::test_support::assert_dsl_round_trip(&next);
    }
    for color in fixture["invalidColors"].as_array().unwrap() {
        assert!(!valid_brush_color(color.as_str().unwrap()));
    }
    for value in [f64::NAN, f64::INFINITY, -0.1, 1.1] {
        assert_eq!(RasterConfigMutation::SetBrushHardness { value }.diff(&base).diff(), &base);
    }
}

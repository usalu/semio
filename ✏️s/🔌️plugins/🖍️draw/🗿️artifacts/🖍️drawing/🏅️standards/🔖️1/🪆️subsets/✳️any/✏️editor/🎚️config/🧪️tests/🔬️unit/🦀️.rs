use super::*;

#[semio_framework_async_macros::async_test]
async fn drawing_config_default_has_no_host_owned_preferences() {
    let config = DrawingConfig::default();
    assert_eq!(config.trace_pointer_generation, 0);
}

#[semio_framework_async_macros::async_test]
async fn drawing_config_dsl_round_trips() {
    let config = DrawingConfig { engagement_input: "Renaming \"layer\"".into(), camera: DrawingCamera { x: 12.0, y: -4.0, zoom: 1.5 }, ..Default::default() };
    store::os_store::test_support::assert_dsl_round_trip(&config);
}

#[semio_framework_async_macros::async_test]
async fn drawing_config_operation_round_trips_and_backwards_restores_snapshot() {
    let base = DrawingConfig::default();
    let operation = DrawingConfigMutation::SetEngagementInput { value: "renaming".into() };
    let forward = operation.diff(&base).diff().clone();
    assert_eq!(forward.engagement_input, "renaming");
    let backwards = operation.inverse(&base);
    assert_eq!(backwards, vec![DrawingConfigMutation::Snapshot { config: base.clone() }]);
    let restored = backwards[0].diff(&forward).diff().clone();
    assert_eq!(restored, base);
}

#[semio_framework_async_macros::async_test]
async fn drawing_config_operation_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&DrawingConfigMutation::Snapshot { config: DrawingConfig::default() });
    store::os_store::test_support::assert_op_line_round_trip(&DrawingConfigMutation::SetEngagementInput { value: "New \"Name\"".into() });
    store::os_store::test_support::assert_op_line_round_trip(&DrawingConfigMutation::SetCamera { camera: DrawingCamera { x: 1.0, y: -2.0, zoom: 3.0 } });
    store::os_store::test_support::assert_op_line_round_trip(&DrawingConfigMutation::SetTracePointerProgress { generation: 7, completed_work: 32, pending_work: 4 });
}

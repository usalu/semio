
use super::*;

#[semio_framework_async_macros::async_test]
async fn drawing_config_default_matches_ui_selectdirect_utility() {
    let config = DrawingConfig::default();
    assert_eq!(config.active_utility_id, "selectDirect");
    assert_eq!(config.locale, "en-US");
}

#[semio_framework_async_macros::async_test]
async fn drawing_config_dsl_round_trips() {
    let config = DrawingConfig { engagement_input: "Renaming \"layer\"".into(), camera: DrawingCamera { x: 12.0, y: -4.0, zoom: 1.5 }, active_utility_id: "pen".into(), locale: "de-DE".into(), ..Default::default() };
    store::os_store::test_support::assert_dsl_round_trip(&config);
}

#[semio_framework_async_macros::async_test]
async fn drawing_config_operation_round_trips_and_backwards_restores_snapshot() {
    let base = DrawingConfig { active_utility_id: "selectDirect".into(), ..Default::default() };
    let operation = DrawingConfigMutation::SetActiveUtility { utility_id: "pen".into() };
    let forward = operation.diff(&base).diff().clone();
    assert_eq!(forward.active_utility_id, "pen");
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
    store::os_store::test_support::assert_op_line_round_trip(&DrawingConfigMutation::SetActiveUtility { utility_id: "pen".into() });
    store::os_store::test_support::assert_op_line_round_trip(&DrawingConfigMutation::SetTracePointerProgress { generation: 7, completed_work: 32, pending_work: 4 });
    store::os_store::test_support::assert_op_line_round_trip(&DrawingConfigMutation::SetLocale { value: "de-DE".into() });
}

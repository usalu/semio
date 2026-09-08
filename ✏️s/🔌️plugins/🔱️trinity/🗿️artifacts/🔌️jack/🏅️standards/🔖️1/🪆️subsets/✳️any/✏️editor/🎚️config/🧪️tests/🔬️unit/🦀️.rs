
use super::*;
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn jack_config_default_has_default_locale() {
    let config = JackConfig::default();
    assert_eq!(config.locale, "en-US");
    assert_eq!(config.camera, Camera::default());
}

#[semio_framework_async_macros::async_test]
async fn jack_config_dsl_round_trips() {
    let mut config = JackConfig { jack_query: "MATCH (a:Piece) RETURN a".into(), editor_selection: Some(JackEditorSelection { start: 3, end: 9 }), ..JackConfig::default() };
    config.lod_mode_by_window.insert("trinity-jack-graph".into(), "compact".into());
    ::store::os_store::test_support::assert_dsl_round_trip(&config);
    ::store::os_store::test_support::assert_dsl_pack_equivalence(&config);
}

#[semio_framework_async_macros::async_test]
async fn jack_config_operation_backwards_restores_prior_snapshot() {
    let base = JackConfig::default();
    let operation = JackConfigMutation::SetActiveFixture(SetActiveFixture { value: "nakagin".into() });
    let next = operation.diff(&base).diff().clone();
    assert_eq!(next.active_fixture_id, "nakagin".to_string());
    let backwards = operation.inverse(&base);
    let restored = backwards[0].diff(&next).diff().clone();
    assert_eq!(restored, base);
}

#[semio_framework_async_macros::async_test]
async fn jack_config_operation_text_round_trips() {
    ::store::os_store::test_support::assert_op_line_round_trip(&JackConfigMutation::SetLodMode(SetLodMode { window_id: "trinity-jack-graph".into(), value: "compact".into() }));
    ::store::os_store::test_support::assert_op_line_round_trip(&JackConfigMutation::SetActiveFixture(SetActiveFixture { value: "nakagin".into() }));
}

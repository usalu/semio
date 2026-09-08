
use super::*;
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn rewriting_config_default_has_default_locale() {
    let config = RewritingConfig::default();
    assert_eq!(config.locale, "en-US");
    assert_eq!(config.before_pane_camera, Camera::default());
}

#[semio_framework_async_macros::async_test]
async fn rewriting_config_dsl_round_trips() {
    let mut config = RewritingConfig { reorganize_epoch: 3, ..RewritingConfig::default() };
    config.lod_mode_by_window.insert("trinity-rewriting-before".into(), "compact".into());
    ::store::os_store::test_support::assert_dsl_round_trip(&config);
    ::store::os_store::test_support::assert_dsl_pack_equivalence(&config);
}

#[semio_framework_async_macros::async_test]
async fn rewriting_config_operation_backwards_restores_prior_snapshot() {
    let base = RewritingConfig::default();
    let operation = RewritingConfigMutation::SetReorganizeEpoch(SetReorganizeEpoch { value: 7 });
    let next = operation.diff(&base).diff().clone();
    assert_eq!(next.reorganize_epoch, 7);
    let backwards = operation.inverse(&base);
    let restored = backwards[0].diff(&next).diff().clone();
    assert_eq!(restored, base);
}

#[semio_framework_async_macros::async_test]
async fn rewriting_config_operation_text_round_trips() {
    ::store::os_store::test_support::assert_op_line_round_trip(&RewritingConfigMutation::SetLodMode(SetLodMode { window_id: "trinity-rewriting-before".into(), value: "compact".into() }));
    ::store::os_store::test_support::assert_op_line_round_trip(&RewritingConfigMutation::SetReorganizeEpoch(SetReorganizeEpoch { value: 4 }));
}


use super::*;

#[semio_framework_async_macros::async_test]
async fn imperative_config_default_is_empty_english() {
    let config = ImperativeConfig::default();
    assert!(config.run_output_json.is_empty());
    assert_eq!(config.locale, "en-US");
}

#[semio_framework_async_macros::async_test]
async fn imperative_config_dsl_round_trips() {
    let config = ImperativeConfig { run_output_json: r#"{"counter":1}"#.into(), locale: "de-DE".into(), contributions_json: "[]".into() };
    store::os_store::test_support::assert_dsl_round_trip(&config);
    store::os_store::test_support::assert_dsl_pack_equivalence(&config);
}

#[semio_framework_async_macros::async_test]
async fn config_operation_snapshot_diff_ignores_base() {
    let base = ImperativeConfig::default();
    let mut snapshot = base.clone();
    snapshot.run_output_json = r#"{"counter":1}"#.into();
    let operation = ImperativeConfigMutation::ReplaceConfig(ReplaceConfig { config: snapshot.clone() });
    assert_eq!(protocol::Mutation::diff(&operation, &base).diff(), &snapshot);
}

#[semio_framework_async_macros::async_test]
async fn config_operation_set_run_output_and_locale_round_trip() {
    store::os_store::test_support::assert_op_line_round_trip(&ImperativeConfigMutation::SetRunOutput(SetRunOutput { json: r#"{"counter":1}"#.into() }));
    store::os_store::test_support::assert_op_line_round_trip(&ImperativeConfigMutation::SetLocale(SetLocale { value: "de-DE".into() }));
}

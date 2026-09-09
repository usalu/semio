use super::*;

#[semio_framework_async_macros::async_test]
async fn imperative_config_default_is_empty_english() {
    let config = ImperativeConfig::default();
    assert!(config.run_output_json.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn imperative_config_dsl_round_trips() {
    let config = ImperativeConfig { run_output_json: r#"{"counter":1}"#.into(), contributions_json: "[]".into() };
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

use crate::editor::procedure::config::*;

#[semio_framework_async_macros::async_test]
async fn imperative_config_default_is_empty_english() {
    let config = ImperativeConfig::default();
    assert!(config.run_output_json.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn imperative_config_dsl_round_trips() {
    let config = ImperativeConfig { run_output_json: r#"{"counter":1}"#.into(), contributions_json: "[]".into() };
    semio_os_store_test_support::assert_dsl_round_trip(&config);
    semio_os_store_test_support::assert_dsl_pack_equivalence(&config);
}

#[semio_framework_async_macros::async_test]
async fn config_replace_diff_names_only_the_changed_fields() {
    let base = ImperativeConfig::default();
    let mut snapshot = base.clone();
    snapshot.run_output_json = r#"{"counter":1}"#.into();
    let operation = ImperativeConfigMutation::ReplaceConfig(ReplaceConfig { config: snapshot.clone() });
    assert_eq!(protocol::ColdMutation::diff(&operation, &base).diff(), &ImperativeConfigDiff { run_output_json: Some(snapshot.run_output_json.clone()), contributions_json: None });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&operation, &base).await;
}

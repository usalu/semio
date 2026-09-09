use super::*;
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn jack_config_dsl_round_trips() {
    let config = JackConfig { jack_query: "MATCH (a:Piece) RETURN a".into() };
    ::store::os_store::test_support::assert_dsl_round_trip(&config);
    ::store::os_store::test_support::assert_dsl_pack_equivalence(&config);
}

#[semio_framework_async_macros::async_test]
async fn jack_config_operation_backwards_restores_prior_snapshot() {
    let base = JackConfig::default();
    let operation = JackConfigMutation::SetQuery(SetQuery { value: "RETURN 1".into() });
    let next = operation.diff(&base).diff().clone();
    assert_eq!(next.jack_query, "RETURN 1".to_string());
    let backwards = operation.inverse(&base);
    let restored = backwards[0].diff(&next).diff().clone();
    assert_eq!(restored, base);
}

#[semio_framework_async_macros::async_test]
async fn jack_config_operation_text_round_trips() {
    ::store::os_store::test_support::assert_op_line_round_trip(&JackConfigMutation::SetQuery(SetQuery { value: "RETURN 1".into() }));
}

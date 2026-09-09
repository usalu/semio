use super::*;

#[semio_framework_async_macros::async_test]
async fn note_config_is_empty_and_round_trips() {
    let config = NoteConfig::default();
    store::os_store::test_support::assert_dsl_pack_equivalence(&config);
    store::os_store::test_support::assert_op_text_binary_equivalence(&NoteConfigMutation::Snapshot { config });
}

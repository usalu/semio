use super::*;

#[semio_framework_async_macros::async_test]
async fn note_config_default_matches_the_pre_migration_runtime_defaults() {
    let config = NoteConfig::default();
    assert_eq!(config.camera, NoteCamera::default());
}

/// 🧮️ B1 Config dsl/pack round-trip law (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE).
#[semio_framework_async_macros::async_test]
async fn note_config_dsl_pack_round_trips() {
    let config = NoteConfig { engagement_input: "Renaming…".into(), camera: NoteCamera { x: 12.5, y: -4.0, zoom: 2.5 } };
    store::os_store::test_support::assert_dsl_pack_equivalence(&config);
}

#[semio_framework_async_macros::async_test]
async fn note_config_operation_text_and_binary_round_trip_every_variant() {
    store::os_store::test_support::assert_op_text_binary_equivalence(&NoteConfigMutation::SetEngagementInput(SetEngagementInput { value: "Renaming…".into() }));
    store::os_store::test_support::assert_op_text_binary_equivalence(&NoteConfigMutation::SetCamera(SetCamera { camera: NoteCamera { x: 4.0, y: 5.0, zoom: 2.0 } }));
}

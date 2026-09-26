use super::*;
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn home_config_dsl_text_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&HomeConfig::default());
}

#[semio_framework_async_macros::async_test]
async fn home_config_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::Snapshot { config: HomeConfig::default() });
    store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::ReplaceDirectoryProjection {
        directory_json: directory_to_json(&store::os_directory::DirectoryReadModel::default()),
        session_binding_sha256: "a".repeat(64),
        authorization_generation: 1,
        receipt_sha256: "b".repeat(64),
    });
}

#[semio_framework_async_macros::async_test]
async fn home_config_default_directory_is_empty() {
    let model = HomeConfig::default().directory().expect("default directory projection");
    assert!(model.spaces.is_empty());
    assert_eq!(model.cursor, 0);
}

#[semio_framework_async_macros::async_test]
async fn directory_projection_round_trip_preserves_documents_and_rejects_corruption() {
    let fixture: pack::JsonValue = pack::parse_json(include_str!("../../🧫️fixtures/📇️projection-persistence-v1/🔣️.json")).expect("language-neutral projection fixture");
    let wire = fixture.get("wire").expect("fixture wire").to_string();
    let model = directory_from_json(&wire).expect("fixture directory projection");
    let document_ids = model.spaces.values().flat_map(|space| space.documents.iter().map(|document| document.document_id.as_str())).collect::<Vec<_>>();
    assert_eq!(document_ids, vec!["document-雪"]);
    let encoded: pack::JsonValue = pack::parse_json(&directory_to_json(&model)).expect("encoded projection JSON");
    assert_eq!(encoded, fixture["wire"]);
    for malformed in fixture["malformed"].as_array().expect("malformed cases") {
        assert!(directory_from_json(malformed.as_str().expect("malformed text")).is_err());
    }
}

#[semio_framework_async_macros::async_test]
async fn home_config_operation_round_trips_via_apply_and_backwards() {
    let config = HomeConfig::default();
    let operation = HomeConfigMutation::ReplaceDirectoryProjection {
        directory_json: directory_to_json(&store::os_directory::DirectoryReadModel::default()),
        session_binding_sha256: "a".repeat(64),
        authorization_generation: 1,
        receipt_sha256: "b".repeat(64),
    };
    let next = operation.diff(&config).diff().clone();
    assert_eq!(next.directory_authorization_generation, 1);
    let backwards = operation.inverse(&config);
    let restored = backwards[0].diff(&next).diff().clone();
    assert_eq!(restored, config);
}

//#region 🧪️RetainedConfigPreparation
#[test]
fn retained_config_cancel_and_cleanup_respect_the_production_grant() {
    use std::io::Write as _;
    use store::ArtifactStoreOneItemPreparation as _;
    let config = HomeConfig::default();
    let mut preparation = HomeConfigPreparation {
        base: None,
        mutation: Some(HomeConfigMutation::ReplaceDirectoryProjection {
            directory_json: config.directory_json,
            session_binding_sha256: "a".repeat(64),
            authorization_generation: 1,
            receipt_sha256: "b".repeat(64),
        }),
        description: None,
        authority: None,
        candidate: None,
        sealed_candidate: None,
        serialized_bytes: None,
        prepared: None,
        checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
        cancelled: false,
        closing: false,
    };
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: HOME_CONFIG_STEP_BYTES };
    preparation.cancel();
    assert!(matches!(preparation.advance(grant).expect("cancelled step"), store::ArtifactStoreOneItemPreparationStep::Blocked));
    preparation.begin_close();
    assert!(matches!(preparation.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 }).expect("undersized close"), store::SnapshotRetirementStep::Blocked));
    assert!(matches!(preparation.close_step(grant).expect("bounded close"), store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes } if released_bytes == HOME_CONFIG_STEP_BYTES));
    assert!(matches!(preparation.close_step(grant).expect("terminal close"), store::SnapshotRetirementStep::Complete));
    assert!(preparation.terminal_is_empty());
    let mut counter = HomeConfigByteCounter { bytes: 0 };
    let maximum = vec![0; HOME_CONFIG_STEP_BYTES];
    assert_eq!(counter.write(&maximum).expect("maximum serialized envelope"), HOME_CONFIG_STEP_BYTES);
    assert!(counter.write(&[0]).is_err());
}
//#endregion 🧪️RetainedConfigPreparation

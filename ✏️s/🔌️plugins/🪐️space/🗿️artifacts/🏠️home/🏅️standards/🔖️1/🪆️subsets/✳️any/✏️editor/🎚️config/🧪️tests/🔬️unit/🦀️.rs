
use super::*;
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn home_config_default_locale_is_english() {
    let config = HomeConfig::default();
    assert_eq!(config.locale, "en-US");
    assert!(config.active_panel_tab.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn home_config_dsl_text_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&HomeConfig::default());
}

#[semio_framework_async_macros::async_test]
async fn home_config_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::Snapshot { config: HomeConfig::default() });
    store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::SetActivePanelTab { tab_id: "tab-1".into() });
    store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::SetLocale { value: "de".into() });
    store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::FoldDirectoryEvent { event_json: "{}".into() });
    store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::ReplaceDirectoryProjection {
        directory_json: directory_to_json(&store::os_directory::DirectoryReadModel::default()),
        session_binding_sha256: "a".repeat(64),
        authorization_generation: 1,
        receipt_sha256: "b".repeat(64),
    });
    store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::SetClient { client_id: "u1".into(), client_name: "Ada".into() });
}

#[semio_framework_async_macros::async_test]
async fn home_config_default_directory_is_empty() {
    let model = HomeConfig::default().directory().expect("default directory projection");
    assert!(model.spaces.is_empty());
    assert_eq!(model.cursor, 0);
}

#[semio_framework_async_macros::async_test]
async fn fold_directory_event_updates_the_read_model() {
    let config = HomeConfig::default();
    let event_json = pack::json!({
        "seq": 1,
        "id": "evt-1",
        "hlc": { "physicalMs": 0, "logical": 0 },
        "actor": { "kind": "user", "id": "user:u1#s1" },
        "spaceId": "sp-1",
        "body": { "kind": "space.created", "spaceId": "sp-1", "name": "Atelier", "spaceKind": "atelier", "visibility": "private", "ownerUserId": "u1" },
        "recordedAtMs": 1000
    })
    .to_string();
    let next = HomeConfigMutation::FoldDirectoryEvent { event_json }.diff(&config).diff().clone();
    let model = next.directory().expect("folded directory projection");
    assert_eq!(model.cursor, 1);
    let space = model.spaces.get("sp-1").expect("space folded");
    assert_eq!(space.view.name, "Atelier");
}

#[semio_framework_async_macros::async_test]
async fn fold_directory_event_ignores_malformed_json() {
    let config = HomeConfig::default();
    let next = HomeConfigMutation::FoldDirectoryEvent { event_json: "not json".into() }.diff(&config).diff().clone();
    assert_eq!(next.directory_json, config.directory_json, "malformed events never panic and never change the model");
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
async fn set_client_updates_identity_fields() {
    let config = HomeConfig::default();
    let next = HomeConfigMutation::SetClient { client_id: "u1".into(), client_name: "Ada".into() }.diff(&config).diff().clone();
    assert_eq!(next.client_id, "u1");
    assert_eq!(next.client_name, "Ada");
}

#[semio_framework_async_macros::async_test]
async fn home_config_operation_round_trips_via_apply_and_backwards() {
    let config = HomeConfig::default();
    let operation = HomeConfigMutation::SetLocale { value: "de".into() };
    let next = operation.diff(&config).diff().clone();
    assert_eq!(next.locale, "de");
    let backwards = operation.inverse(&config);
    let restored = backwards[0].diff(&next).diff().clone();
    assert_eq!(restored, config);
}

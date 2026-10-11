
use super::*;

#[semio_framework_async_macros::async_test]
async fn default_config_is_private_with_no_members_or_presence() {
    let config = SpaceIndexConfig::default();
    assert_eq!(config.visibility, "private");
    assert!(config.members.is_empty());
    assert!(config.indexed_artifacts.is_empty());
    assert!(config.presence.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn presence_for_splits_the_csv() {
    let config = SpaceIndexConfig { presence: vec![SpaceIndexArtifactPresence { artifact_id: "artifact-1".into(), actors_csv: "user:1,user:2".into() }], ..Default::default() };
    assert_eq!(config.presence_for("artifact-1"), vec!["user:1", "user:2"]);
    assert!(config.presence_for("ghost").is_empty());
}

#[semio_framework_async_macros::async_test]
async fn config_dsl_round_trips() {
    let config = SpaceIndexConfig {
        visibility: "public".into(),
        members: vec![SpaceIndexMember { user_id: "u-1".into(), email: "a@example.com".into(), display_name: "Alice".into(), role: "author".into() }],
        indexed_artifacts: Vec::new(),
        presence: vec![SpaceIndexArtifactPresence { artifact_id: "artifact-1".into(), actors_csv: "user:1".into() }],
    };
    store::os_store::test_support::assert_dsl_round_trip(&config);
}

#[semio_framework_async_macros::async_test]
async fn directory_projection_replaces_only_the_directory_slice_and_inverse_restores() {
    let base = SpaceIndexConfig { presence: vec![SpaceIndexArtifactPresence { artifact_id: "artifact-1".into(), actors_csv: "user:1".into() }], ..Default::default() };
    let mutation = SpaceIndexConfigMutation::ReplaceDirectoryProjection(DirectoryProjectionReplacement { projection: SpaceIndexDirectoryProjection { visibility: "public".into(), ..Default::default() } });
    let outcome = mutation.diff(&base);
    assert_eq!(outcome.diff(), &SpaceIndexConfigDiff { visibility: Some("public".into()), ..Default::default() });
    let forward = protocol::apply_diff(outcome.diff(), &base).expect("the projection applies");
    assert_eq!(forward.presence, base.presence, "the directory fold never touches the live presence rows");
    let backwards = mutation.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(backwards, vec![SpaceIndexConfigMutation::ReplaceDirectoryProjection(DirectoryProjectionReplacement { projection: SpaceIndexDirectoryProjection { visibility: "private".into(), ..Default::default() } })]);
    assert_eq!(protocol::apply_diff(backwards[0].diff(&forward).diff(), &forward).expect("the inverse applies"), base);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn artifact_presence_sets_clears_and_inverts_exactly() {
    let base = SpaceIndexConfig { presence: vec![SpaceIndexArtifactPresence { artifact_id: "artifact-1".into(), actors_csv: "user:1".into() }], ..Default::default() };
    for mutation in [
        SpaceIndexConfigMutation::SetArtifactPresence(ArtifactPresenceSetting { artifact_id: "artifact-1".into(), actors_csv: "user:2".into() }),
        SpaceIndexConfigMutation::SetArtifactPresence(ArtifactPresenceSetting { artifact_id: "artifact-2".into(), actors_csv: "user:3".into() }),
        SpaceIndexConfigMutation::ClearArtifactPresence(ArtifactPresenceClearing { artifact_id: "artifact-1".into() }),
    ] {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_presence_sets_and_clears_at_the_sorted_middle_slot() {
    let row = |id: &str| SpaceIndexArtifactPresence { artifact_id: id.into(), actors_csv: "user:1".into() };
    let base = SpaceIndexConfig { presence: vec![row("artifact-1"), row("artifact-3")], ..Default::default() };
    let set = SpaceIndexConfigMutation::SetArtifactPresence(ArtifactPresenceSetting { artifact_id: "artifact-2".into(), actors_csv: "user:1".into() });
    let diff = <SpaceIndexConfigMutation as protocol::Mutation<SpaceIndexConfig>>::diff(&set, &base).into_parts().0;
    let inserted = diff.presence.as_ref().expect("a new presence row").inserted.iter().map(|entry| (entry.index, entry.row.artifact_id.as_str())).collect::<Vec<_>>();
    assert_eq!(inserted, vec![(1, "artifact-2")]);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&set, &base).await;
    let full = SpaceIndexConfig { presence: vec![row("artifact-1"), row("artifact-2"), row("artifact-3")], ..Default::default() };
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SpaceIndexConfigMutation::ClearArtifactPresence(ArtifactPresenceClearing { artifact_id: "artifact-2".into() }), &full).await;
}

#[semio_framework_async_macros::async_test]
async fn config_mutation_op_text_round_trips() {
    store::os_store::test_support::assert_op_line_round_trip(&SpaceIndexConfigMutation::ReplaceDirectoryProjection(DirectoryProjectionReplacement { projection: SpaceIndexDirectoryProjection::default() }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceIndexConfigMutation::SetArtifactPresence(ArtifactPresenceSetting { artifact_id: "artifact-1".into(), actors_csv: "user:1".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceIndexConfigMutation::ClearArtifactPresence(ArtifactPresenceClearing { artifact_id: "artifact-1".into() }));
}

/// 🧾️ The payload-record variants keep the former named variants' externally tagged wire byte for byte, checked against an
/// independent serde_json reading of the exact historical literals.
#[test]
fn config_mutation_wire_is_identical_to_the_former_named_variants() {
    let fixture = [
        (SpaceIndexConfigMutation::SetArtifactPresence(ArtifactPresenceSetting { artifact_id: "artifact-1".into(), actors_csv: "user:2".into() }), r#"{"SetArtifactPresence":{"artifact_id":"artifact-1","actors_csv":"user:2"}}"#),
        (SpaceIndexConfigMutation::ClearArtifactPresence(ArtifactPresenceClearing { artifact_id: "artifact-1".into() }), r#"{"ClearArtifactPresence":{"artifact_id":"artifact-1"}}"#),
    ];
    for (mutation, literal) in fixture {
        let json = semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&mutation));
        let encoded: serde_json::Value = serde_json::from_str(&json).expect("the encoded wire is valid json");
        let historical: serde_json::Value = serde_json::from_str(literal).expect("the historical literal is valid json");
        assert_eq!(encoded, historical);
        let parsed = semio_framework_pack_json::parse(literal, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the historical literal parses");
        assert_eq!(<SpaceIndexConfigMutation as semio_framework_value::FromValue>::from_value(parsed).expect("the historical literal decodes"), mutation);
    }
}

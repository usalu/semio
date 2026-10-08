
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
    let mutation = SpaceIndexConfigMutation::ReplaceDirectoryProjection { projection: SpaceIndexDirectoryProjection { visibility: "public".into(), ..Default::default() } };
    let outcome = mutation.diff(&base);
    assert_eq!(outcome.diff(), &SpaceIndexConfigDiff { visibility: Some("public".into()), ..Default::default() });
    let forward = protocol::apply_diff(outcome.diff(), &base).expect("the projection applies");
    assert_eq!(forward.presence, base.presence, "the directory fold never touches the live presence rows");
    let backwards = mutation.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(backwards, vec![SpaceIndexConfigMutation::ReplaceDirectoryProjection { projection: SpaceIndexDirectoryProjection { visibility: "private".into(), ..Default::default() } }]);
    assert_eq!(protocol::apply_diff(backwards[0].diff(&forward).diff(), &forward).expect("the inverse applies"), base);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn artifact_presence_sets_clears_and_inverts_exactly() {
    let base = SpaceIndexConfig { presence: vec![SpaceIndexArtifactPresence { artifact_id: "artifact-1".into(), actors_csv: "user:1".into() }], ..Default::default() };
    for mutation in [
        SpaceIndexConfigMutation::SetArtifactPresence { artifact_id: "artifact-1".into(), actors_csv: "user:2".into() },
        SpaceIndexConfigMutation::SetArtifactPresence { artifact_id: "artifact-2".into(), actors_csv: "user:3".into() },
        SpaceIndexConfigMutation::ClearArtifactPresence { artifact_id: "artifact-1".into() },
    ] {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_presence_sets_and_clears_at_the_sorted_middle_slot() {
    let row = |id: &str| SpaceIndexArtifactPresence { artifact_id: id.into(), actors_csv: "user:1".into() };
    let base = SpaceIndexConfig { presence: vec![row("artifact-1"), row("artifact-3")], ..Default::default() };
    let set = SpaceIndexConfigMutation::SetArtifactPresence { artifact_id: "artifact-2".into(), actors_csv: "user:1".into() };
    let diff = <SpaceIndexConfigMutation as protocol::Mutation<SpaceIndexConfig>>::diff(&set, &base).into_parts().0;
    let inserted = diff.presence.as_ref().expect("a new presence row").inserted.iter().map(|entry| (entry.index, entry.row.artifact_id.as_str())).collect::<Vec<_>>();
    assert_eq!(inserted, vec![(1, "artifact-2")]);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&set, &base).await;
    let full = SpaceIndexConfig { presence: vec![row("artifact-1"), row("artifact-2"), row("artifact-3")], ..Default::default() };
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SpaceIndexConfigMutation::ClearArtifactPresence { artifact_id: "artifact-2".into() }, &full).await;
}

#[semio_framework_async_macros::async_test]
async fn config_mutation_op_text_round_trips() {
    store::os_store::test_support::assert_op_line_round_trip(&SpaceIndexConfigMutation::ReplaceDirectoryProjection { projection: SpaceIndexDirectoryProjection::default() });
    store::os_store::test_support::assert_op_line_round_trip(&SpaceIndexConfigMutation::SetArtifactPresence { artifact_id: "artifact-1".into(), actors_csv: "user:1".into() });
    store::os_store::test_support::assert_op_line_round_trip(&SpaceIndexConfigMutation::ClearArtifactPresence { artifact_id: "artifact-1".into() });
}

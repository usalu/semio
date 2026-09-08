
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
async fn config_mutation_snapshot_replaces_wholesale_and_inverse_restores() {
    let base = SpaceIndexConfig::default();
    let next = SpaceIndexConfig { visibility: "public".into(), ..Default::default() };
    let mutation = SpaceIndexConfigMutation::Snapshot { config: next.clone() };
    let forward = mutation.diff(&base).diff().clone();
    assert_eq!(forward, next);
    let backwards = mutation.inverse(&base);
    assert_eq!(backwards, vec![SpaceIndexConfigMutation::Snapshot { config: base.clone() }]);
    let restored = backwards[0].diff(&forward).diff().clone();
    assert_eq!(restored, base);
}

#[semio_framework_async_macros::async_test]
async fn config_mutation_op_text_round_trips() {
    store::os_store::test_support::assert_op_line_round_trip(&SpaceIndexConfigMutation::Snapshot { config: SpaceIndexConfig::default() });
}

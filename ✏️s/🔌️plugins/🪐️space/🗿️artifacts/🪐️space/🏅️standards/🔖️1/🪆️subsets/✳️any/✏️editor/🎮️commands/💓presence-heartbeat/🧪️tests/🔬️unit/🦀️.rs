
use super::*;

fn empty_doc() -> (SSpaceSnapshot, semio_framework_plugin::HistoryView) {
    (SSpaceSnapshot::default(), semio_framework_plugin::HistoryView::empty())
}

#[semio_framework_async_macros::async_test]
async fn heartbeat_sets_presence_for_a_new_artifact() {
    let (snapshot, history) = empty_doc();
    let doc = ArtifactView::new(&snapshot, &history);
    let config_snapshot = SpaceIndexConfig::default();
    let cfg = ConfigView { snapshot: &config_snapshot, window: None };
    let result = handle(&PresenceHeartbeat { artifact_id: "artifact-1".into(), actors_csv: "user:1,user:2".into() }, &doc, &cfg).expect("heartbeat");
    assert_eq!(result.config_mutations.len(), 1);
    let SpaceIndexConfigMutation::Snapshot { config } = &result.config_mutations[0];
    assert_eq!(config.presence_for("artifact-1"), vec!["user:1", "user:2"]);
}

#[semio_framework_async_macros::async_test]
async fn heartbeat_replaces_an_existing_artifacts_presence() {
    let (snapshot, history) = empty_doc();
    let doc = ArtifactView::new(&snapshot, &history);
    let seeded = SpaceIndexConfig { presence: vec![SpaceIndexArtifactPresence { artifact_id: "artifact-1".into(), actors_csv: "user:1".into() }], ..Default::default() };
    let cfg = ConfigView { snapshot: &seeded, window: None };
    let result = handle(&PresenceHeartbeat { artifact_id: "artifact-1".into(), actors_csv: "user:2".into() }, &doc, &cfg).expect("heartbeat");
    let SpaceIndexConfigMutation::Snapshot { config } = &result.config_mutations[0];
    assert_eq!(config.presence.len(), 1, "replaces, does not append, an existing artifact's row");
    assert_eq!(config.presence_for("artifact-1"), vec!["user:2"]);
}

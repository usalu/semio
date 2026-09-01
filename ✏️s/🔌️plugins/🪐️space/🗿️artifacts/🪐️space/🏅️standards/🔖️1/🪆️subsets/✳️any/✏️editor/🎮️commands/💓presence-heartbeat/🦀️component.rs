//! 💓️ SpaceIndexEditor commands command — `presence-heartbeat`. Config-only: replaces the live
//! actor list for one artifact (contract §C1's `DirectoryStreamMessage::Presence { spaceId,
//! documentId, surface, actors }`, folded per artifact rather than per `(documentId, surface)` pair —
//! the space table shows one row per artifact, so every surface/document of that artifact is merged
//! into one peer list for the "presence" column, worker-brief task 1). The shell dispatches this once
//! per presence stream message it forwards for this space.

use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexArtifactPresence, SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "presence-heartbeat")]
pub struct PresenceHeartbeat {
    pub artifact_id: String,
    pub actors_csv: String,
}

pub async fn handle(payload: &PresenceHeartbeat, _doc: &ArtifactView<'_, SSpaceSnapshot>, cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    let mut presence = cfg.snapshot.presence.clone();
    match presence.iter_mut().find(|row| row.artifact_id == payload.artifact_id) {
        Some(row) => row.actors_csv = payload.actors_csv.clone(),
        None => presence.push(SpaceIndexArtifactPresence { artifact_id: payload.artifact_id.clone(), actors_csv: payload.actors_csv.clone() }),
    }
    let next = SpaceIndexConfig { presence, ..cfg.snapshot.clone() };
    Ok(Emit { config_mutations: vec![SpaceIndexConfigMutation::Snapshot { config: next }], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    async fn empty_doc() -> (SSpaceSnapshot, semio_framework_plugin::HistoryView) {
        (SSpaceSnapshot::default(), semio_framework_plugin::HistoryView::empty())
    }

    #[semio_framework_async_macros::async_test]
    async fn heartbeat_sets_presence_for_a_new_artifact() {
        let (snapshot, history) = empty_doc();
        let doc = ArtifactView::new(&snapshot, &history);
        let config_snapshot = SpaceIndexConfig::default();
        let cfg = ConfigView { snapshot: &config_snapshot };
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
        let cfg = ConfigView { snapshot: &seeded };
        let result = handle(&PresenceHeartbeat { artifact_id: "artifact-1".into(), actors_csv: "user:2".into() }, &doc, &cfg).expect("heartbeat");
        let SpaceIndexConfigMutation::Snapshot { config } = &result.config_mutations[0];
        assert_eq!(config.presence.len(), 1, "replaces, does not append, an existing artifact's row");
        assert_eq!(config.presence_for("artifact-1"), vec!["user:2"]);
    }
}
//#endregion 🧪️Tests

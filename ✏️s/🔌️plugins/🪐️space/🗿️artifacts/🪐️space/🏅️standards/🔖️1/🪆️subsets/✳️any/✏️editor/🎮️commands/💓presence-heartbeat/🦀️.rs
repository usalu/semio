//! 💓️ SpaceIndexEditor commands command — `presence-heartbeat`. Config-only: replaces the live
//! actor list for one artifact (contract §C1's `DirectoryStreamMessage::Presence { spaceId,
//! documentId, surface, actors }`, folded per artifact rather than per `(documentId, surface)` pair —
//! the space table shows one row per artifact, so every surface/document of that artifact is merged
//! into one peer list for the "presence" column, worker-brief task 1). The shell dispatches this once
//! per presence stream message it forwards for this space.

use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexArtifactPresence, SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "presence-heartbeat")]
pub struct PresenceHeartbeat {
    pub artifact_id: String,
    pub actors_csv: String,
}

pub fn handle(payload: &PresenceHeartbeat, _doc: &ArtifactView<'_, SSpaceSnapshot>, cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

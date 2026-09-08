//! 🗑️️ SpaceIndexEditor commands command — `delete-artifact`.

use crate::standards::v1::subsets::any::schema::mutations::{delete_artifact, SSpaceMutation};
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

/// 🗑️️ The real, immediate delete — never dispatched directly from a row's "Delete" affordance
/// (that dispatches `❔request-delete-artifact` first, which opens the `deleteArtifact` confirm
/// dialog; the dialog's own submit re-dispatches THIS command, worker-brief task 2). Kept as its own
/// undecorated command so a caller that already confirmed out-of-band (tests, scripted flows) is
/// never forced through the dialog.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "delete-artifact")]
pub struct DeleteArtifact {
    pub id: String,
}

pub fn handle(payload: &DeleteArtifact, _doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![delete_artifact(payload.id.clone())]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

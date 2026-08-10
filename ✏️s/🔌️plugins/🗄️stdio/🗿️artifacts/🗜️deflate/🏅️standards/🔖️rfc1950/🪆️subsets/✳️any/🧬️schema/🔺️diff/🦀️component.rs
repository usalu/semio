//! 🔺️ DeflateDiff — sparse replace-snapshot diff.

use crate::artifacts::deflate::DeflateSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.deflate`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.deflate.diff")]
pub struct DeflateDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<DeflateSnapshot>,
}

impl MutationDiff<DeflateSnapshot> for DeflateDiff {
    fn apply(&self, base: &DeflateSnapshot) -> DeflateSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &DeflateSnapshot) -> DeflateDiff {
    DeflateDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

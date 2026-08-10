//! 🔺️ PlyDiff — sparse replace-snapshot diff.

use crate::artifacts::ply::PlySnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.ply`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ply.diff")]
pub struct PlyDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<PlySnapshot>,
}

impl MutationDiff<PlySnapshot> for PlyDiff {
    fn apply(&self, base: &PlySnapshot) -> PlySnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &PlySnapshot) -> PlyDiff {
    PlyDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

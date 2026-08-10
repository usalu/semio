//! 🔺️ LasDiff — sparse replace-snapshot diff.

use crate::artifacts::las::LasSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.las`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.las.diff")]
pub struct LasDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<LasSnapshot>,
}

impl MutationDiff<LasSnapshot> for LasDiff {
    fn apply(&self, base: &LasSnapshot) -> LasSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &LasSnapshot) -> LasDiff {
    LasDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

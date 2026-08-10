//! 🔺️ StlDiff — sparse replace-snapshot diff.

use crate::artifacts::stl::StlSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.stl`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.stl.diff")]
pub struct StlDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<StlSnapshot>,
}

impl MutationDiff<StlSnapshot> for StlDiff {
    fn apply(&self, base: &StlSnapshot) -> StlSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &StlSnapshot) -> StlDiff {
    StlDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

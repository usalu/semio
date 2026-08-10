//! 🔺️ DwgDiff — sparse replace-snapshot diff.

use crate::artifacts::dwg::DwgSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.dwg`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dwg.diff")]
pub struct DwgDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<DwgSnapshot>,
}

impl MutationDiff<DwgSnapshot> for DwgDiff {
    fn apply(&self, base: &DwgSnapshot) -> DwgSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &DwgSnapshot) -> DwgDiff {
    DwgDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

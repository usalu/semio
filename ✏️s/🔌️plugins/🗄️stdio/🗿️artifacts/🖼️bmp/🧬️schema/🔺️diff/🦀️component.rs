//! 🔺️ BmpDiff — sparse replace-snapshot diff.

use crate::artifacts::bmp::BmpSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.bmp`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bmp.diff")]
pub struct BmpDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<BmpSnapshot>,
}

impl MutationDiff<BmpSnapshot> for BmpDiff {
    fn apply(&self, base: &BmpSnapshot) -> BmpSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &BmpSnapshot) -> BmpDiff {
    BmpDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

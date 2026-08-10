//! 🔺️ PngDiff — sparse replace-snapshot diff.

use crate::artifacts::png::PngSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.png`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.png.diff")]
pub struct PngDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<PngSnapshot>,
}

impl MutationDiff<PngSnapshot> for PngDiff {
    fn apply(&self, base: &PngSnapshot) -> PngSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &PngSnapshot) -> PngDiff {
    PngDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

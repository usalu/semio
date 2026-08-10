//! 🔺️ TiffDiff — sparse replace-snapshot diff.

use crate::artifacts::tiff::TiffSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.tiff`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.tiff.diff")]
pub struct TiffDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<TiffSnapshot>,
}

impl MutationDiff<TiffSnapshot> for TiffDiff {
    fn apply(&self, base: &TiffSnapshot) -> TiffSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &TiffSnapshot) -> TiffDiff {
    TiffDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

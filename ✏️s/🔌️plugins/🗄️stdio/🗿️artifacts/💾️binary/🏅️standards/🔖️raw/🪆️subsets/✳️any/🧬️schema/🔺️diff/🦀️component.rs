//! 🔺️ BinaryDiff — sparse replace-snapshot diff.

use crate::artifacts::binary::BinarySnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.binary`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.binary.diff")]
pub struct BinaryDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<BinarySnapshot>,
}

impl MutationDiff<BinarySnapshot> for BinaryDiff {
    fn apply(&self, base: &BinarySnapshot) -> BinarySnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &BinarySnapshot) -> BinaryDiff {
    BinaryDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

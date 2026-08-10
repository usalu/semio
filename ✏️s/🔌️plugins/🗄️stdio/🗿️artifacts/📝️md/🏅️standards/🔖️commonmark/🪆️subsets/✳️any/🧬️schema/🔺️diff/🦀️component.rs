//! 🔺️ MdDiff — sparse replace-snapshot diff.

use crate::artifacts::md::MdSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.md`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.md.diff")]
pub struct MdDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<MdSnapshot>,
}

impl MutationDiff<MdSnapshot> for MdDiff {
    fn apply(&self, base: &MdSnapshot) -> MdSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &MdSnapshot) -> MdDiff {
    MdDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

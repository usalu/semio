//! 🔺️ TxtDiff — sparse replace-snapshot diff.

use crate::artifacts::txt::TxtSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.txt`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.txt.diff")]
pub struct TxtDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<TxtSnapshot>,
}

impl MutationDiff<TxtSnapshot> for TxtDiff {
    fn apply(&self, base: &TxtSnapshot) -> TxtSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &TxtSnapshot) -> TxtDiff {
    TxtDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

//! 🔺️ JpgDiff — sparse replace-snapshot diff.

use crate::artifacts::jpg::JpgSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.jpg`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.jpg.diff")]
pub struct JpgDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<JpgSnapshot>,
}

impl MutationDiff<JpgSnapshot> for JpgDiff {
    fn apply(&self, base: &JpgSnapshot) -> JpgSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &JpgSnapshot) -> JpgDiff {
    JpgDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

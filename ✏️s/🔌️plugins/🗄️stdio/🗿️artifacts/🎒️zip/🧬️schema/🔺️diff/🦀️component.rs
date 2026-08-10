//! 🔺️ ZipDiff — sparse replace-snapshot diff.

use crate::artifacts::zip::ZipSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.zip`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.zip.diff")]
pub struct ZipDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<ZipSnapshot>,
}

impl MutationDiff<ZipSnapshot> for ZipDiff {
    fn apply(&self, base: &ZipSnapshot) -> ZipSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &ZipSnapshot) -> ZipDiff {
    ZipDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

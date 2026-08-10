//! 🔺️ XlsxDiff — sparse replace-snapshot diff.

use crate::artifacts::xlsx::XlsxSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.xlsx`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xlsx.diff")]
pub struct XlsxDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<XlsxSnapshot>,
}

impl MutationDiff<XlsxSnapshot> for XlsxDiff {
    fn apply(&self, base: &XlsxSnapshot) -> XlsxSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &XlsxSnapshot) -> XlsxDiff {
    XlsxDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

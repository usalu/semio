//! 🔺️ DocxDiff — sparse replace-snapshot diff.

use crate::artifacts::docx::DocxSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.docx`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.docx.diff")]
pub struct DocxDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<DocxSnapshot>,
}

impl MutationDiff<DocxSnapshot> for DocxDiff {
    fn apply(&self, base: &DocxSnapshot) -> DocxSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &DocxSnapshot) -> DocxDiff {
    DocxDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

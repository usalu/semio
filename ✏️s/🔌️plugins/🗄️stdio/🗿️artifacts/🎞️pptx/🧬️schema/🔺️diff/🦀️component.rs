//! 🔺️ PptxDiff — sparse replace-snapshot diff.

use crate::artifacts::pptx::PptxSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.pptx`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pptx.diff")]
pub struct PptxDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<PptxSnapshot>,
}

impl MutationDiff<PptxSnapshot> for PptxDiff {
    fn apply(&self, base: &PptxSnapshot) -> PptxSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &PptxSnapshot) -> PptxDiff {
    PptxDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

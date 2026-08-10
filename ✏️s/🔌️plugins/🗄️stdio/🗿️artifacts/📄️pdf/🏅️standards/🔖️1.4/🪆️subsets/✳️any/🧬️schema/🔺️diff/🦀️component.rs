//! 🔺️ PdfDiff — sparse replace-snapshot diff.

use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.pdf`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf.diff")]
pub struct PdfDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<PdfSnapshot>,
}

impl MutationDiff<PdfSnapshot> for PdfDiff {
    fn apply(&self, base: &PdfSnapshot) -> PdfSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &PdfSnapshot) -> PdfDiff {
    PdfDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

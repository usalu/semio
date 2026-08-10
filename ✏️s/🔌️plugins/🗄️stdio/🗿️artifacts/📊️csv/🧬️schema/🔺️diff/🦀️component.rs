//! 🔺️ CsvDiff — sparse replace-snapshot diff.

use crate::artifacts::csv::CsvSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.csv`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.csv.diff")]
pub struct CsvDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<CsvSnapshot>,
}

impl MutationDiff<CsvSnapshot> for CsvDiff {
    fn apply(&self, base: &CsvSnapshot) -> CsvSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &CsvSnapshot) -> CsvDiff {
    CsvDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

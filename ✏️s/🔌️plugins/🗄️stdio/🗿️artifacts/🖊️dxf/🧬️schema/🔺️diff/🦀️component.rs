//! 🔺️ DxfDiff — sparse replace-snapshot diff.

use crate::artifacts::dxf::DxfSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.dxf`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dxf.diff")]
pub struct DxfDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<DxfSnapshot>,
}

impl MutationDiff<DxfSnapshot> for DxfDiff {
    fn apply(&self, base: &DxfSnapshot) -> DxfSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &DxfSnapshot) -> DxfDiff {
    DxfDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

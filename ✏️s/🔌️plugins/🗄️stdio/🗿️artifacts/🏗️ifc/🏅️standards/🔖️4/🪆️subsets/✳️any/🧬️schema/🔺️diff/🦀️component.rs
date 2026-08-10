//! 🔺️ IfcDiff — sparse replace-snapshot diff.

use crate::artifacts::ifc::IfcSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.ifc`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc.diff")]
pub struct IfcDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<IfcSnapshot>,
}

impl MutationDiff<IfcSnapshot> for IfcDiff {
    fn apply(&self, base: &IfcSnapshot) -> IfcSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &IfcSnapshot) -> IfcDiff {
    IfcDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

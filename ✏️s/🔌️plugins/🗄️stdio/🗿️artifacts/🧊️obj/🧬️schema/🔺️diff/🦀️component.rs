//! 🔺️ ObjDiff — sparse replace-snapshot diff.

use crate::artifacts::obj::ObjSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.obj`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.obj.diff")]
pub struct ObjDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<ObjSnapshot>,
}

impl MutationDiff<ObjSnapshot> for ObjDiff {
    fn apply(&self, base: &ObjSnapshot) -> ObjSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &ObjSnapshot) -> ObjDiff {
    ObjDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

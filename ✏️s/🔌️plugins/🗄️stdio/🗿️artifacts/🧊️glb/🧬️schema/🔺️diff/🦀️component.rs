//! 🔺️ GlbDiff — sparse replace-snapshot diff.

use crate::artifacts::glb::GlbSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.glb`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.glb.diff")]
pub struct GlbDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<GlbSnapshot>,
}

impl MutationDiff<GlbSnapshot> for GlbDiff {
    fn apply(&self, base: &GlbSnapshot) -> GlbSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &GlbSnapshot) -> GlbDiff {
    GlbDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

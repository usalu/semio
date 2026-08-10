//! 🔺️ GifDiff — sparse replace-snapshot diff.

use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.gif`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gif.diff")]
pub struct GifDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<GifSnapshot>,
}

impl MutationDiff<GifSnapshot> for GifDiff {
    fn apply(&self, base: &GifSnapshot) -> GifSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &GifSnapshot) -> GifDiff {
    GifDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

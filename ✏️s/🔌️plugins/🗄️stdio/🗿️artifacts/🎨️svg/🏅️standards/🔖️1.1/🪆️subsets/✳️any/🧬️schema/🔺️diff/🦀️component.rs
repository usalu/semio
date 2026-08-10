//! 🔺️ SvgDiff — sparse replace-snapshot diff.

use crate::artifacts::svg::SvgSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.svg`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.svg.diff")]
pub struct SvgDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<SvgSnapshot>,
}

impl MutationDiff<SvgSnapshot> for SvgDiff {
    fn apply(&self, base: &SvgSnapshot) -> SvgSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &SvgSnapshot) -> SvgDiff {
    SvgDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

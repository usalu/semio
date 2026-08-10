//! 🔺️ StepDiff — sparse replace-snapshot diff.

use crate::artifacts::step::StepSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.step`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.step.diff")]
pub struct StepDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<StepSnapshot>,
}

impl MutationDiff<StepSnapshot> for StepDiff {
    fn apply(&self, base: &StepSnapshot) -> StepSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &StepSnapshot) -> StepDiff {
    StepDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

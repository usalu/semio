//! 🔺️ BcfDiff — sparse replace-snapshot diff.

use crate::artifacts::bcf::BcfSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.bcf`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bcf.diff")]
pub struct BcfDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<BcfSnapshot>,
}

impl MutationDiff<BcfSnapshot> for BcfDiff {
    fn apply(&self, base: &BcfSnapshot) -> BcfSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &BcfSnapshot) -> BcfDiff {
    BcfDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

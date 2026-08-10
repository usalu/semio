//! 🔺️ XmlDiff — sparse replace-snapshot diff.

use crate::artifacts::xml::XmlSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.xml`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xml.diff")]
pub struct XmlDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<XmlSnapshot>,
}

impl MutationDiff<XmlSnapshot> for XmlDiff {
    fn apply(&self, base: &XmlSnapshot) -> XmlSnapshot {
        self.snapshot.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
    }
}

/// 🧩 Builds a set-snapshot diff.
pub fn diff_set_snapshot(snapshot: &XmlSnapshot) -> XmlDiff {
    XmlDiff { snapshot: Some(snapshot.clone()) }
}
//#endregion 🔖️Diff

//! Puzzle5d mutation — `ChangePartAnchor`: changes whether a part keeps its stored plane or resets to default XY.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `change-part-anchor` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-part-anchor")]
pub struct ChangePartAnchor {
    pub id: String,
    pub new_anchor: crate::artifacts::puzzle5d::Puzzle5dPartAnchor,
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ChangePartAnchor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "part", kind: "change-part-anchor", record: "ChangedPartAnchor" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> Puzzle5dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change part \"{}\" anchor", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_part_anchor(id: String, new_anchor: crate::artifacts::puzzle5d::Puzzle5dPartAnchor) -> Puzzle5dMutation {
    Puzzle5dMutation::ChangePartAnchor(ChangePartAnchor { id, new_anchor })
}

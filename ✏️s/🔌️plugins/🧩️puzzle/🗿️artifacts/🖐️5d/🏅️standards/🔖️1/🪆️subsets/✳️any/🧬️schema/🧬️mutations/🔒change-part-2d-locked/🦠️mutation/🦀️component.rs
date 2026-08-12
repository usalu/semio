//! Puzzle5d mutation — `ChangePart2dLocked`: changes a part's 2D-projection locked flag.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `change-part-2d-locked` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-part-2d-locked")]
pub struct ChangePart2dLocked {
    pub id: String,
    pub new_locked: Option<bool>,
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ChangePart2dLocked {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "part", kind: "change-part-2d-locked", record: "ChangedPart2dLocked" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> Puzzle5dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change part \"{}\" 2d locked", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_part_2d_locked(id: String, new_locked: Option<bool>) -> Puzzle5dMutation {
    Puzzle5dMutation::ChangePart2dLocked(ChangePart2dLocked { id, new_locked })
}

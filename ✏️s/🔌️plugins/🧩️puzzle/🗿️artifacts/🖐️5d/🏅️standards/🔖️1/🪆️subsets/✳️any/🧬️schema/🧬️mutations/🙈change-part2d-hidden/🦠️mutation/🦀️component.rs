//! Puzzle5d mutation — `ChangePart2dHidden`: changes a part's 2D-projection hidden flag.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `change-part2d-hidden` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-part2d-hidden")]
pub struct ChangePart2dHidden {
    pub id: String,
    pub new_hidden: Option<bool>,
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ChangePart2dHidden {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "part", kind: "change-part2d-hidden", record: "ChangedPart2dHidden" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> Puzzle5dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change part \"{}\" 2d hidden", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_part_2d_hidden(id: String, new_hidden: Option<bool>) -> Puzzle5dMutation {
    Puzzle5dMutation::ChangePart2dHidden(ChangePart2dHidden { id, new_hidden })
}

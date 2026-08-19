//! Puzzle5d mutation — `EditPart2dText`: replaces a part's 2D-projection authored display text.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `edit-part2d-text` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "edit-part2d-text")]
pub struct EditPart2dText {
    pub id: String,
    pub new_text: Option<String>,
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for EditPart2dText {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "part", kind: "edit-part2d-text", record: "EditedPart2dText" };

    async fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Edit part \"{}\" 2d text", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn edit_part_2d_text(id: String, new_text: Option<String>) -> Puzzle5dMutation {
    Puzzle5dMutation::EditPart2dText(EditPart2dText { id, new_text })
}

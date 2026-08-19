//! ➕ Puzzle5d mutation — `AddPartGrip`: attaches a new rim grip to a part.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::{Puzzle5dGrip, Puzzle5dSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➕ `add-part-grip` payload — owner part id + new grip payload at an optional FINAL-state
/// `index` (`None` appends). A duplicate `grip.id` on the same part is a no-op.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-part-grip")]
pub struct AddPartGrip {
    pub part_id: String,
    #[dsl(block)]
    pub grip: Puzzle5dGrip,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn add_part_grip(part_id: String, grip: Puzzle5dGrip, index: Option<usize>) -> Puzzle5dMutation {
    Puzzle5dMutation::AddPartGrip(AddPartGrip { part_id, grip, index })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for AddPartGrip {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "part-grip", kind: "add-part-grip", record: "AddedPartGrip" };

    async fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Add grip \"{}\" to part \"{}\"", self.grip.id, self.part_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.part_id.clone(), self.grip.id.clone()]
    }
}
//#endregion 🔖️Mutation

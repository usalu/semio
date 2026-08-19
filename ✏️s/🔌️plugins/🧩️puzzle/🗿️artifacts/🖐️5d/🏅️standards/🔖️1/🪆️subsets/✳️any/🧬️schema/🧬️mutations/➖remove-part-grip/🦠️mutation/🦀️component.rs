//! ➖ Puzzle5d mutation — `RemovePartGrip`: detaches a rim grip from a part (captures cascade —
//! any fastener whose `source`/`target` referenced this grip is severed too).
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➖ `remove-part-grip` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-part-grip")]
pub struct RemovePartGrip {
    pub part_id: String,
    pub grip_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn remove_part_grip(part_id: String, grip_id: String) -> Puzzle5dMutation {
    Puzzle5dMutation::RemovePartGrip(RemovePartGrip { part_id, grip_id })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for RemovePartGrip {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "part-grip", kind: "remove-part-grip", record: "RemovedPartGrip" };

    async fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Remove grip \"{}\" from part \"{}\"", self.grip_id, self.part_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.part_id.clone(), self.grip_id.clone()]
    }
}
//#endregion 🔖️Mutation

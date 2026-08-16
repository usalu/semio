//! 🔌 Puzzle5d mutation — `ReplacePartGrip`: whole-value swap of one grip's presentation fields
//! (kind/2d/3d together, one property-panel gesture).
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::{Puzzle5dGrip, Puzzle5dSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔌 `replace-part-grip` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-part-grip")]
pub struct ReplacePartGrip {
    pub part_id: String,
    pub grip_id: String,
    #[dsl(block)]
    pub new_grip: Puzzle5dGrip,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_part_grip(part_id: String, grip_id: String, new_grip: Puzzle5dGrip) -> Puzzle5dMutation {
    Puzzle5dMutation::ReplacePartGrip(ReplacePartGrip { part_id, grip_id, new_grip })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ReplacePartGrip {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "part-grip", kind: "replace-part-grip", record: "ReplacedPartGrip" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace grip \"{}\" on part \"{}\"", self.grip_id, self.part_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.part_id.clone(), self.grip_id.clone()]
    }
}
//#endregion 🔖️Mutation

//! Puzzle5d mutation — `MovePart3d`: absolute reposition of a part's 3D-projection origin.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `move-part3d` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "move-part3d")]
pub struct MovePart3d {
    pub id: String,
    pub new_origin: [f64; 3],
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for MovePart3d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "part", kind: "move-part3d", record: "MovedPart3d" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move part \"{}\" (3d)", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_part_3d(id: String, new_origin: [f64; 3]) -> Puzzle5dMutation {
    Puzzle5dMutation::MovePart3d(MovePart3d { id, new_origin })
}

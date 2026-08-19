//! 🧭 Block5d mutation — `MoveGrip3d`: a grip's 3D position + facing direction together.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧭 `move-grip-3d` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "move-grip-3d")]
pub struct MoveGrip3d {
    pub id: String,
    pub new_position: [f64; 3],
    pub new_direction: [f64; 3],
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn move_grip_3d(id: String, new_position: [f64; 3], new_direction: [f64; 3]) -> Block5dMutation {
    Block5dMutation::MoveGrip3d(MoveGrip3d { id, new_position, new_direction })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for MoveGrip3d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "grip", kind: "move-grip3d", record: "MovedGrip3d" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Move grip \"{}\" (3D)", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

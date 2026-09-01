//! 📍 Block3d mutation — `MoveVortex`: a vortex's position + facing direction together.

use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexTemplate};
use crate::artifacts::block3d::diff::{Block3dDiff, Block3dVorticesDelta, Block3dVorticesPatch, Block3dVorticesPatchEntry};
use crate::artifacts::block3d::mutations::Block3dMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📍 `move-vortex` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "move-vortex")]
pub struct MoveVortex {
    pub id: String,
    pub new_position: [f64; 3],
    pub new_direction: [f64; 3],
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn move_vortex(id: String, new_position: [f64; 3], new_direction: [f64; 3]) -> Block3dMutation {
    Block3dMutation::MoveVortex(MoveVortex { id, new_position, new_direction })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for MoveVortex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "vortex", kind: "move-vortex", record: "MovedVortex" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Move vortex \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

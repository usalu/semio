//! 🎬 Block5d mutation — `MoveCamera3d`: the 3D-projection window camera's position + look-at target together.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Mutation
/// 🎬 `move-camera3d` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "move-camera3d")]
pub struct MoveCamera3d {
    pub new_position: [f64; 3],
    pub new_target: [f64; 3],
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn move_camera3d(new_position: [f64; 3], new_target: [f64; 3]) -> Block5dMutation {
    Block5dMutation::MoveCamera3d(MoveCamera3d { new_position, new_target })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for MoveCamera3d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "camera3d", kind: "move-camera3d", record: "MovedCamera3d" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Move 3D camera to {:?}", self.new_position)
    }
}
//#endregion 🔖️Mutation

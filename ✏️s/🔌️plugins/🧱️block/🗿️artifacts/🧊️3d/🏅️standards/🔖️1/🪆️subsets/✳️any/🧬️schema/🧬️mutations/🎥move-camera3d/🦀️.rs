//! 🎥 Block3d mutation — `MoveCamera3d`: the 3D world camera's position + look-at target together.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Mutation
/// 🎥 `move-camera3d` payload.
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
pub async fn move_camera3d(new_position: [f64; 3], new_target: [f64; 3]) -> Block3dMutation {
    Block3dMutation::MoveCamera3d(MoveCamera3d { new_position, new_target })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for MoveCamera3d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "camera3d", kind: "move-camera3d", record: "MovedCamera3d" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Move camera to {:?}", self.new_position)
    }
}
//#endregion 🔖️Mutation

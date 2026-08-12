//! 🎬 Block5d mutation — `MoveCamera3d`: the 3D-projection window camera's position + look-at target together.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🎬 `move-camera3d` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "move-camera3d")]
pub struct MoveCamera3d {
    pub new_position: [f64; 3],
    pub new_target: [f64; 3],
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_camera3d(new_position: [f64; 3], new_target: [f64; 3]) -> Block5dMutation {
    Block5dMutation::MoveCamera3d(MoveCamera3d { new_position, new_target })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for MoveCamera3d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "camera3d", kind: "move-camera3d", record: "MovedCamera3d" };

    fn diff(&self, base: &Block5dSnapshot) -> Block5dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move 3D camera to {:?}", self.new_position)
    }
}
//#endregion 🔖️Mutation

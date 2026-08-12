//! 🎥️ Block2d mutation — `MoveCamera2d`: the 2D board camera's `x`/`y` pan position.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🎥️ `move-camera2d` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "move-camera2d")]
pub struct MoveCamera2d {
    pub new_x: f64,
    pub new_y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_camera2d(new_x: f64, new_y: f64) -> Block2dMutation {
    Block2dMutation::MoveCamera2d(MoveCamera2d { new_x, new_y })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for MoveCamera2d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "camera2d", kind: "move-camera2d", record: "MovedCamera2d" };

    fn diff(&self, base: &Block2dSnapshot) -> Block2dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move camera to ({}, {})", self.new_x, self.new_y)
    }
}
//#endregion 🔖️Mutation

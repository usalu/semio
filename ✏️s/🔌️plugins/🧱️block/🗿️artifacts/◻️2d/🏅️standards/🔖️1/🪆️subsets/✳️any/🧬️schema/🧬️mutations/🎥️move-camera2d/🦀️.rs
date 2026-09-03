//! 🎥️ Block2d mutation — `MoveCamera2d`: the 2D board camera's `x`/`y` pan position.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Mutation
/// 🎥️ `move-camera2d` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "move-camera2d")]
pub struct MoveCamera2d {
    pub new_x: f64,
    pub new_y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn move_camera2d(new_x: f64, new_y: f64) -> Block2dMutation {
    Block2dMutation::MoveCamera2d(MoveCamera2d { new_x, new_y })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for MoveCamera2d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "camera2d", kind: "move-camera2d", record: "MovedCamera2d" };

    async fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Move camera to ({}, {})", self.new_x, self.new_y)
    }
}
//#endregion 🔖️Mutation

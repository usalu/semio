//! 🎥 Block5d mutation — `MoveCamera2d`: the 2D-projection window camera's `x`/`y` pan position.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Mutation
/// 🎥 `move-camera2d` payload.
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
pub async fn move_camera2d(new_x: f64, new_y: f64) -> Block5dMutation {
    Block5dMutation::MoveCamera2d(MoveCamera2d { new_x, new_y })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for MoveCamera2d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "camera2d", kind: "move-camera2d", record: "MovedCamera2d" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Move 2D camera to ({}, {})", self.new_x, self.new_y)
    }
}
//#endregion 🔖️Mutation

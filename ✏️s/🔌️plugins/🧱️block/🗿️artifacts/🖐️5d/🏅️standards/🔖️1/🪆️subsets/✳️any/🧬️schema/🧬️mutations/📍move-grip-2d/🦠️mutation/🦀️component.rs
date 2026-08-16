//! 📍 Block5d mutation — `MoveGrip2d`: a grip's 2D polar position (`angle`+`radius2d` together).
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📍 `move-grip-2d` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "move-grip-2d")]
pub struct MoveGrip2d {
    pub id: String,
    pub new_angle: f64,
    pub new_radius_2d: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_grip_2d(id: String, new_angle: f64, new_radius_2d: f64) -> Block5dMutation {
    Block5dMutation::MoveGrip2d(MoveGrip2d { id, new_angle, new_radius_2d })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for MoveGrip2d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "grip", kind: "move-grip2d", record: "MovedGrip2d" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move grip \"{}\" (2D)", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

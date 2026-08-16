//! 📍️ Block2d mutation — `MoveHandle`: a rim-handle's polar position (`angle`+`radius` together).
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📍️ `move-handle` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "move-handle")]
pub struct MoveHandle {
    pub id: String,
    pub new_angle: f64,
    pub new_radius: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_handle(id: String, new_angle: f64, new_radius: f64) -> Block2dMutation {
    Block2dMutation::MoveHandle(MoveHandle { id, new_angle, new_radius })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for MoveHandle {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "handle", kind: "move-handle", record: "MovedHandle" };

    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move handle \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

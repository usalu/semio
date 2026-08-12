//! 📏 Block5d mutation — `ResizeGrip3d`: a grip's 3D `radius3d`.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📏 `resize-grip-3d` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "resize-grip-3d")]
pub struct ResizeGrip3d {
    pub id: String,
    pub new_radius_3d: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_grip_3d(id: String, new_radius_3d: f64) -> Block5dMutation {
    Block5dMutation::ResizeGrip3d(ResizeGrip3d { id, new_radius_3d })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ResizeGrip3d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "resize", entity: "grip", kind: "resize-grip-3d", record: "ResizedGrip3d" };

    fn diff(&self, base: &Block5dSnapshot) -> Block5dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Resize grip \"{}\" (3D)", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

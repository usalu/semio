//! 🔍️ Block2d mutation — `ScaleCamera2d`: the 2D board camera's `zoom` level.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔍️ `scale-camera2d` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "scale-camera2d")]
pub struct ScaleCamera2d {
    pub new_zoom: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn scale_camera2d(new_zoom: f64) -> Block2dMutation {
    Block2dMutation::ScaleCamera2d(ScaleCamera2d { new_zoom })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for ScaleCamera2d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "camera2d", kind: "scale-camera2d", record: "ScaledCamera2d" };

    fn diff(&self, base: &Block2dSnapshot) -> Block2dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Scale camera to {}", self.new_zoom)
    }
}
//#endregion 🔖️Mutation

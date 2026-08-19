//! 🔍 Block5d mutation — `ScaleCamera2d`: the 2D-projection window camera's `zoom` level.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔍 `scale-camera2d` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "scale-camera2d")]
pub struct ScaleCamera2d {
    pub new_zoom: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn scale_camera2d(new_zoom: f64) -> Block5dMutation {
    Block5dMutation::ScaleCamera2d(ScaleCamera2d { new_zoom })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ScaleCamera2d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "camera2d", kind: "scale-camera2d", record: "ScaledCamera2d" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Scale 2D camera to {}", self.new_zoom)
    }
}
//#endregion 🔖️Mutation

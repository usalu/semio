//! 🔎 Block5d mutation — `ScaleCamera3d`: the 3D-projection window camera's `zoom` level.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔎 `scale-camera3d` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "scale-camera3d")]
pub struct ScaleCamera3d {
    pub new_zoom: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn scale_camera3d(new_zoom: f64) -> Block5dMutation {
    Block5dMutation::ScaleCamera3d(ScaleCamera3d { new_zoom })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ScaleCamera3d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "camera3d", kind: "scale-camera3d", record: "ScaledCamera3d" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Scale 3D camera to {}", self.new_zoom)
    }
}
//#endregion 🔖️Mutation

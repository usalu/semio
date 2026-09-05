//! 🔍 Block3d mutation — `ScaleCamera3d`: the 3D world camera's `zoom` level.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Mutation
/// 🔍 `scale-camera3d` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "scale-camera3d")]
pub struct ScaleCamera3d {
    pub new_zoom: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn scale_camera3d(new_zoom: f64) -> Block3dMutation {
    Block3dMutation::ScaleCamera3d(ScaleCamera3d { new_zoom })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for ScaleCamera3d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "camera3d", kind: "scale-camera3d", record: "ScaledCamera3d" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Scale camera to {}", self.new_zoom)
    }
}
//#endregion 🔖️Mutation

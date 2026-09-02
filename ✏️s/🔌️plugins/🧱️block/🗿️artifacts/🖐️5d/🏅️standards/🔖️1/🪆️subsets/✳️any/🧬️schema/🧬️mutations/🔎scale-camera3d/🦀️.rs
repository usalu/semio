//! 🔎 Block5d mutation — `ScaleCamera3d`: the 3D-projection window camera's `zoom` level.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Mutation
/// 🔎 `scale-camera3d` payload.
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
pub async fn scale_camera3d(new_zoom: f64) -> Block5dMutation {
    Block5dMutation::ScaleCamera3d(ScaleCamera3d { new_zoom })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ScaleCamera3d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "camera3d", kind: "scale-camera3d", record: "ScaledCamera3d" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Scale 3D camera to {}", self.new_zoom)
    }
}
//#endregion 🔖️Mutation

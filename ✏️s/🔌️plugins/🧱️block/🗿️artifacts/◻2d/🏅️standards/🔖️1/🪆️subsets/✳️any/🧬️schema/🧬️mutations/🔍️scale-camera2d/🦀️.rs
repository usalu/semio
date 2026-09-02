//! 🔍️ Block2d mutation — `ScaleCamera2d`: the 2D board camera's `zoom` level.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Mutation
/// 🔍️ `scale-camera2d` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "scale-camera2d")]
pub struct ScaleCamera2d {
    pub new_zoom: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn scale_camera2d(new_zoom: f64) -> Block2dMutation {
    Block2dMutation::ScaleCamera2d(ScaleCamera2d { new_zoom })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for ScaleCamera2d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "camera2d", kind: "scale-camera2d", record: "ScaledCamera2d" };

    async fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Scale camera to {}", self.new_zoom)
    }
}
//#endregion 🔖️Mutation

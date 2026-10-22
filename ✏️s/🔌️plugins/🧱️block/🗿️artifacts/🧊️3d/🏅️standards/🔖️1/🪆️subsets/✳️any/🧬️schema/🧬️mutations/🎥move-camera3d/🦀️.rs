//! 🎥 Block3d mutation — `MoveCamera3d`: the 3D world camera's position + look-at target together.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::Block3dDiff;

//#region 🔖️Mutation
/// 🎥 `move-camera3d` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "move-camera3d")]
pub struct MoveCamera3d {
    pub new_position: [f64; 3],
    pub new_target: [f64; 3],
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_camera3d<Op: From<crate::move_camera3d::MoveCamera3d>>(new_position: [f64; 3], new_target: [f64; 3]) -> Op {
    Op::from(MoveCamera3d { new_position, new_target })
}

impl<Op> protocol::MutationKind<Block3dSnapshot, Op> for MoveCamera3d where Op: protocol::Mutation<Block3dSnapshot, Diff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::move_camera3d::MoveCamera3d> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "camera3d", kind: "move-camera3d", record: "MovedCamera3d" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move camera to {:?}", self.new_position), &format!("Kamera nach {:?} verschieben", self.new_position))
    }
}

impl<Op> protocol::ColdMutationKind<Block3dSnapshot, Op> for MoveCamera3d where Op: protocol::ColdMutation<Block3dSnapshot, ReferenceDiff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::move_camera3d::MoveCamera3d> {
    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        crate::move_camera3d::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::move_camera3d::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts

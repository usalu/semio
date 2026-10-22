//! 🔍 Block3d mutation — `ScaleCamera3d`: the 3D world camera's `zoom` level.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::Block3dDiff;

//#region 🔖️Mutation
/// 🔍 `scale-camera3d` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "scale-camera3d")]
pub struct ScaleCamera3d {
    pub new_zoom: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn scale_camera3d<Op: From<crate::scale_camera3d::ScaleCamera3d>>(new_zoom: f64) -> Op {
    Op::from(ScaleCamera3d { new_zoom })
}

impl<Op> protocol::MutationKind<Block3dSnapshot, Op> for ScaleCamera3d where Op: protocol::Mutation<Block3dSnapshot, Diff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::scale_camera3d::ScaleCamera3d> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "camera3d", kind: "scale-camera3d", record: "ScaledCamera3d" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale camera to {}", self.new_zoom), &format!("Kamerazoom auf {} setzen", self.new_zoom))
    }
}

impl<Op> protocol::ColdMutationKind<Block3dSnapshot, Op> for ScaleCamera3d where Op: protocol::ColdMutation<Block3dSnapshot, ReferenceDiff = semio_s_artifact_block_3d_diff::Block3dDiff> + From<crate::scale_camera3d::ScaleCamera3d> {
    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        crate::scale_camera3d::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::scale_camera3d::inverse::inverse(self, base)?
    
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

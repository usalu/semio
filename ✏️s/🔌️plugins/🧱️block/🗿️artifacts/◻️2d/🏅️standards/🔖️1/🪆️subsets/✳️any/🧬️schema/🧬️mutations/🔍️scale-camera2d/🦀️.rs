//! 🔍️ Block2d mutation — `ScaleCamera2d`: the 2D board camera's `zoom` level.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::Block2dDiff;

//#region 🔖️Mutation
/// 🔍️ `scale-camera2d` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "scale-camera2d")]
pub struct ScaleCamera2d {
    pub new_zoom: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn scale_camera2d<Op: From<crate::scale_camera2d::ScaleCamera2d>>(new_zoom: f64) -> Op {
    Op::from(ScaleCamera2d { new_zoom })
}

impl<Op> protocol::MutationKind<Block2dSnapshot, Op> for ScaleCamera2d where Op: protocol::Mutation<Block2dSnapshot, Diff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::scale_camera2d::ScaleCamera2d> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "camera2d", kind: "scale-camera2d", record: "ScaledCamera2d" };

    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        crate::scale_camera2d::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::scale_camera2d::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale camera to {}", self.new_zoom), &format!("Kamerazoom auf {} setzen", self.new_zoom))
    }
}
//#endregion 🔖️Mutation

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts

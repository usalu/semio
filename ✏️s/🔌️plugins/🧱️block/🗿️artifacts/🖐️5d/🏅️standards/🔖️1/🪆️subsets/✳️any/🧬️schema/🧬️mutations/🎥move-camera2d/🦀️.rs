//! 🎥 Block5d mutation — `MoveCamera2d`: the 2D-projection window camera's `x`/`y` pan position.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Mutation
/// 🎥 `move-camera2d` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "move-camera2d")]
pub struct MoveCamera2d {
    pub new_x: f64,
    pub new_y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_camera2d<Op: From<crate::move_camera2d::MoveCamera2d>>(new_x: f64, new_y: f64) -> Op {
    Op::from(MoveCamera2d { new_x, new_y })
}

impl<Op> protocol::MutationKind<Block5dSnapshot, Op> for MoveCamera2d where Op: protocol::Mutation<Block5dSnapshot, Diff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::move_camera2d::MoveCamera2d> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "camera2d", kind: "move-camera2d", record: "MovedCamera2d" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move 2D camera to ({}, {})", self.new_x, self.new_y), &format!("2D-Kamera nach ({}, {}) verschieben", self.new_x, self.new_y))
    }
}

impl<Op> protocol::ColdMutationKind<Block5dSnapshot, Op> for MoveCamera2d where Op: protocol::ColdMutation<Block5dSnapshot, ReferenceDiff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::move_camera2d::MoveCamera2d> {
    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        crate::move_camera2d::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::move_camera2d::inverse::inverse(self, base)?
    
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

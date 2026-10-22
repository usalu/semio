//! 📏 Block5d mutation — `ResizeGrip3d`: a grip's 3D `radius3d`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Mutation
/// 📏 `resize-grip-3d` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "resize-grip-3d")]
pub struct ResizeGrip3d {
    pub id: String,
    pub new_radius_3d: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_grip_3d<Op: From<crate::resize_grip_3d::ResizeGrip3d>>(id: String, new_radius_3d: f64) -> Op {
    Op::from(ResizeGrip3d { id, new_radius_3d })
}

impl<Op> protocol::MutationKind<Block5dSnapshot, Op> for ResizeGrip3d where Op: protocol::Mutation<Block5dSnapshot, Diff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::resize_grip_3d::ResizeGrip3d> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "resize", entity: "grip", kind: "resize-grip3d", record: "ResizedGrip3d" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Resize grip \"{}\" (3D)", self.id), &format!("Größe von Griff \"{}\" (3D) ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Block5dSnapshot, Op> for ResizeGrip3d where Op: protocol::ColdMutation<Block5dSnapshot, ReferenceDiff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::resize_grip_3d::ResizeGrip3d> {
    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        crate::resize_grip_3d::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::resize_grip_3d::inverse::inverse(self, base)?
    
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

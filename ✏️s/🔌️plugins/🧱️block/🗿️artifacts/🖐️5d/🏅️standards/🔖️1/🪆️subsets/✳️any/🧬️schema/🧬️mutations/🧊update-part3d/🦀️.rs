//! 🧊 Block5d mutation — `UpdatePart3d`: the whole 2-field 3D-projection pose facet atomically (orientation quaternion + scale vector, always edited together in a 3D pose gizmo).

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Mutation
/// 🧊 `update-part-3d` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "update-part-3d")]
pub struct UpdatePart3d {
    pub new_orientation: Option<[f64; 4]>,
    pub new_scale: Option<[f64; 3]>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_part_3d<Op: From<crate::update_part_3d::UpdatePart3d>>(new_orientation: Option<[f64; 4]>, new_scale: Option<[f64; 3]>) -> Op {
    Op::from(UpdatePart3d { new_orientation, new_scale })
}

impl<Op> protocol::MutationKind<Block5dSnapshot, Op> for UpdatePart3d where Op: protocol::Mutation<Block5dSnapshot, Diff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::update_part_3d::UpdatePart3d> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "part-3d", kind: "update-part3d", record: "UpdatedPart3d" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        crate::update_part_3d::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::update_part_3d::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Update part 3D pose", "3D-Lage des Bauteils aktualisieren")
    }
}
//#endregion 🔖️Mutation

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts

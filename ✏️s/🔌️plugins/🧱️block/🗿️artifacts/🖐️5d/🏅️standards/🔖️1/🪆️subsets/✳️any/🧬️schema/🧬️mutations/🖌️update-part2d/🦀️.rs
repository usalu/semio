//! 🖌️ Block5d mutation — `UpdatePart2d`: the whole 2D-projection presentation facet atomically — mirrors block2d's `update-presentation` (same shape, same no-identity-field reasoning).

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Mutation
/// 🖌️ `update-part-2d` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "update-part-2d")]
pub struct UpdatePart2d {
    pub new_shape: Option<String>,
    pub new_radius: Option<f64>,
    pub new_width: Option<f64>,
    pub new_height: Option<f64>,
    pub new_color: Option<String>,
    pub new_icon_kind: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_part_2d<Op: From<crate::update_part_2d::UpdatePart2d>>(new_shape: Option<String>, new_radius: Option<f64>, new_width: Option<f64>, new_height: Option<f64>, new_color: Option<String>, new_icon_kind: Option<String>) -> Op {
    Op::from(UpdatePart2d { new_shape, new_radius, new_width, new_height, new_color, new_icon_kind })
}

impl<Op> protocol::MutationKind<Block5dSnapshot, Op> for UpdatePart2d where Op: protocol::Mutation<Block5dSnapshot, Diff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::update_part_2d::UpdatePart2d> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "part-2d", kind: "update-part2d", record: "UpdatedPart2d" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Update part 2D presentation", "2D-Darstellung des Bauteils aktualisieren")
    }
}

impl<Op> protocol::ColdMutationKind<Block5dSnapshot, Op> for UpdatePart2d where Op: protocol::ColdMutation<Block5dSnapshot, ReferenceDiff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::update_part_2d::UpdatePart2d> {
    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        crate::update_part_2d::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::update_part_2d::inverse::inverse(self, base)?
    
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

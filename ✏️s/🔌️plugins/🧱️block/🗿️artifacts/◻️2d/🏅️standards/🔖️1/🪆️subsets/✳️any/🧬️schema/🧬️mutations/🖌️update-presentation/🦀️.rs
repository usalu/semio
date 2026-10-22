//! 🖌️ Block2d mutation — `UpdatePresentation`: the whole rim-presentation facet atomically (shape/radius/width/height/color/iconKind are edited together in the shape inspector — see report).

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::Block2dDiff;

//#region 🔖️Mutation
/// 🖌️ `update-presentation` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "update-presentation")]
pub struct UpdatePresentation {
    pub new_shape: Option<String>,
    pub new_radius: Option<f64>,
    pub new_width: Option<f64>,
    pub new_height: Option<f64>,
    pub new_color: Option<String>,
    pub new_icon_kind: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_presentation<Op: From<crate::update_presentation::UpdatePresentation>>(new_shape: Option<String>, new_radius: Option<f64>, new_width: Option<f64>, new_height: Option<f64>, new_color: Option<String>, new_icon_kind: Option<String>) -> Op {
    Op::from(UpdatePresentation { new_shape, new_radius, new_width, new_height, new_color, new_icon_kind })
}

impl<Op> protocol::MutationKind<Block2dSnapshot, Op> for UpdatePresentation where Op: protocol::Mutation<Block2dSnapshot, Diff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::update_presentation::UpdatePresentation> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "presentation", kind: "update-presentation", record: "UpdatedPresentation" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Update presentation", "Präsentation aktualisieren")
    }
}

impl<Op> protocol::ColdMutationKind<Block2dSnapshot, Op> for UpdatePresentation where Op: protocol::ColdMutation<Block2dSnapshot, ReferenceDiff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::update_presentation::UpdatePresentation> {
    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        crate::update_presentation::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::update_presentation::inverse::inverse(self, base)?
    
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

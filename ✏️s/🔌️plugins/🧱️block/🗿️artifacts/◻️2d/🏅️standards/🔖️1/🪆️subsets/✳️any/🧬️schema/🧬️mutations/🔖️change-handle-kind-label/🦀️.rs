//! 🔖️ Block2d mutation — `ChangeHandleKindLabel`: a handle-kind catalog row's `label`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::Block2dDiff;

//#region 🔖️Mutation
/// 🔖️ `change-handle-kind-label` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-handle-kind-label")]
pub struct ChangeHandleKindLabel {
    pub id: String,
    pub new_label: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_handle_kind_label<Op: From<crate::change_handle_kind_label::ChangeHandleKindLabel>>(id: String, new_label: String) -> Op {
    Op::from(ChangeHandleKindLabel { id, new_label })
}

impl<Op> protocol::MutationKind<Block2dSnapshot, Op> for ChangeHandleKindLabel where Op: protocol::Mutation<Block2dSnapshot, Diff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::change_handle_kind_label::ChangeHandleKindLabel> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "handle-kind", kind: "change-handle-kind-label", record: "ChangedHandleKindLabel" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change handle kind \"{}\" label to \"{}\"", self.id, self.new_label), &format!("Beschriftung von Griffart \"{}\" auf \"{}\" ändern", self.id, self.new_label))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Block2dSnapshot, Op> for ChangeHandleKindLabel where Op: protocol::ColdMutation<Block2dSnapshot, ReferenceDiff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::change_handle_kind_label::ChangeHandleKindLabel> {
    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        crate::change_handle_kind_label::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_handle_kind_label::inverse::inverse(self, base)?
    
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

//! 🎫 Block5d mutation — `ChangeGripKindLabel`: a grip-kind catalog row's `label`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Mutation
/// 🎫 `change-grip-kind-label` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-grip-kind-label")]
pub struct ChangeGripKindLabel {
    pub id: String,
    pub new_label: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_grip_kind_label<Op: From<crate::change_grip_kind_label::ChangeGripKindLabel>>(id: String, new_label: String) -> Op {
    Op::from(ChangeGripKindLabel { id, new_label })
}

impl<Op> protocol::MutationKind<Block5dSnapshot, Op> for ChangeGripKindLabel where Op: protocol::Mutation<Block5dSnapshot, Diff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::change_grip_kind_label::ChangeGripKindLabel> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "grip-kind", kind: "change-grip-kind-label", record: "ChangedGripKindLabel" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change grip kind \"{}\" label to \"{}\"", self.id, self.new_label), &format!("Beschriftung von Griffart \"{}\" auf \"{}\" ändern", self.id, self.new_label))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Block5dSnapshot, Op> for ChangeGripKindLabel where Op: protocol::ColdMutation<Block5dSnapshot, ReferenceDiff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::change_grip_kind_label::ChangeGripKindLabel> {
    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        crate::change_grip_kind_label::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_grip_kind_label::inverse::inverse(self, base)?
    
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

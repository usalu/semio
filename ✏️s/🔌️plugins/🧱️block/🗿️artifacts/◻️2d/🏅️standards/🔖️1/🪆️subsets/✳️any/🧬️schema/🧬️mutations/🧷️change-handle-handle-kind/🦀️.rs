//! 🧷️ Block2d mutation — `ChangeHandleHandleKind`: a rim-handle's `handleKind` catalog reference (rebind).

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::Block2dDiff;

//#region 🔖️Mutation
/// 🧷️ `change-handle-handle-kind` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-handle-handle-kind")]
pub struct ChangeHandleHandleKind {
    pub id: String,
    pub new_handle_kind: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_handle_handle_kind<Op: From<crate::change_handle_handle_kind::ChangeHandleHandleKind>>(id: String, new_handle_kind: String) -> Op {
    Op::from(ChangeHandleHandleKind { id, new_handle_kind })
}

impl<Op> protocol::MutationKind<Block2dSnapshot, Op> for ChangeHandleHandleKind where Op: protocol::Mutation<Block2dSnapshot, Diff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::change_handle_handle_kind::ChangeHandleHandleKind> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "handle", kind: "change-handle-handle-kind", record: "ChangedHandleHandleKind" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change handle \"{}\" handle kind to \"{}\"", self.id, self.new_handle_kind), &format!("Griffart von Griff \"{}\" auf \"{}\" ändern", self.id, self.new_handle_kind))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Block2dSnapshot, Op> for ChangeHandleHandleKind where Op: protocol::ColdMutation<Block2dSnapshot, ReferenceDiff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::change_handle_handle_kind::ChangeHandleHandleKind> {
    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        crate::change_handle_handle_kind::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_handle_handle_kind::inverse::inverse(self, base)?
    
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

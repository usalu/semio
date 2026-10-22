//! 🗑️ Block2d mutation — `DeleteHandleKind`: a handle-kind catalog row.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::Block2dDiff;

//#region 🔖️Mutation
/// 🗑️ `delete-handle-kind` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "delete-handle-kind")]
pub struct DeleteHandleKind {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_handle_kind<Op: From<crate::delete_handle_kind::DeleteHandleKind>>(id: String) -> Op {
    Op::from(DeleteHandleKind { id })
}

impl<Op> protocol::MutationKind<Block2dSnapshot, Op> for DeleteHandleKind where Op: protocol::Mutation<Block2dSnapshot, Diff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::create_handle_kind::CreateHandleKind> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "handle-kind", kind: "delete-handle-kind", record: "DeletedHandleKind" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete handle kind \"{}\"", self.id), &format!("Griffart \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Block2dSnapshot, Op> for DeleteHandleKind where Op: protocol::ColdMutation<Block2dSnapshot, ReferenceDiff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::create_handle_kind::CreateHandleKind> {
    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        crate::delete_handle_kind::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::delete_handle_kind::inverse::inverse(self, base)?
    
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

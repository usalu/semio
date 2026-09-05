//! 🗑️ Block2d mutation — `DeleteHandleKind`: a handle-kind catalog row.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::diff::{Block2dDiff, Block2dHandleKindsDelta};
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Mutation
/// 🗑️ `delete-handle-kind` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "delete-handle-kind")]
pub struct DeleteHandleKind {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_handle_kind(id: String) -> Block2dMutation {
    Block2dMutation::DeleteHandleKind(DeleteHandleKind { id })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for DeleteHandleKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "handle-kind", kind: "delete-handle-kind", record: "DeletedHandleKind" };

    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete handle kind \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

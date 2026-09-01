//! ❌️ Block2d mutation — `DeleteHandle`: a rim-handle template.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::diff::{Block2dDiff, Block2dHandlesDelta};
use crate::artifacts::block2d::mutations::Block2dMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ❌️ `delete-handle` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-handle")]
pub struct DeleteHandle {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn delete_handle(id: String) -> Block2dMutation {
    Block2dMutation::DeleteHandle(DeleteHandle { id })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for DeleteHandle {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "handle", kind: "delete-handle", record: "DeletedHandle" };

    async fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete handle \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

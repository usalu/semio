//! ✒️ Block2d mutation — `RenameHandleKind`: a handle-kind catalog row's `name`.

use crate::artifacts::block2d::{Block2dHandleKind, Block2dSnapshot};
use crate::artifacts::block2d::diff::{Block2dDiff, Block2dHandleKindsDelta, Block2dHandleKindsPatch, Block2dHandleKindsPatchEntry};
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Mutation
/// ✒️ `rename-handle-kind` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "rename-handle-kind")]
pub struct RenameHandleKind {
    pub id: String,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn rename_handle_kind(id: String, new_name: String) -> Block2dMutation {
    Block2dMutation::RenameHandleKind(RenameHandleKind { id, new_name })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for RenameHandleKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "handle-kind", kind: "rename-handle-kind", record: "RenamedHandleKind" };

    async fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Rename handle kind \"{}\" to \"{}\"", self.id, self.new_name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

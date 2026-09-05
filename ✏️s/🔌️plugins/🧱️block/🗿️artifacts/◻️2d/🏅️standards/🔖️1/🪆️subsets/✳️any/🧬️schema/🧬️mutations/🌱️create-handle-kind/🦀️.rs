//! 🌱️ Block2d mutation — `CreateHandleKind`: a new handle-kind catalog row.

use crate::artifacts::block2d::{Block2dHandleKind, Block2dSnapshot};
use crate::artifacts::block2d::diff::{Block2dDiff, Block2dHandleKindsDelta};
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Mutation
/// 🌱️ `create-handle-kind` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-handle-kind")]
pub struct CreateHandleKind {
    #[dsl(block)]
    pub handle_kind: Block2dHandleKind,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_handle_kind(handle_kind: Block2dHandleKind) -> Block2dMutation {
    Block2dMutation::CreateHandleKind(CreateHandleKind { handle_kind })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for CreateHandleKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "handle-kind", kind: "create-handle-kind", record: "CreatedHandleKind" };

    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create handle kind \"{}\"", self.handle_kind.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.handle_kind.id.clone()]
    }
}
//#endregion 🔖️Mutation

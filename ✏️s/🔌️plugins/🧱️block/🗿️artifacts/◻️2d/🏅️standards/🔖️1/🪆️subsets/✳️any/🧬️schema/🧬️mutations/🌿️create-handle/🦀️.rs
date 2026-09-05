//! 🌿️ Block2d mutation — `CreateHandle`: a new rim-handle template.

use crate::artifacts::block2d::{Block2dHandleTemplate, Block2dSnapshot};
use crate::artifacts::block2d::diff::{Block2dDiff, Block2dHandlesDelta};
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Mutation
/// 🌿️ `create-handle` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-handle")]
pub struct CreateHandle {
    #[dsl(block)]
    pub handle: Block2dHandleTemplate,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_handle(handle: Block2dHandleTemplate) -> Block2dMutation {
    Block2dMutation::CreateHandle(CreateHandle { handle })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for CreateHandle {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "handle", kind: "create-handle", record: "CreatedHandle" };

    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create handle \"{}\"", self.handle.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.handle.id.clone()]
    }
}
//#endregion 🔖️Mutation

//! 🌿️ Block2d mutation — `CreateHandle`: a new rim-handle template.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dHandleTemplate;
use crate::artifacts::block2d::Block2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌿️ `create-handle` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-handle")]
pub struct CreateHandle {
    #[dsl(block)]
    pub handle: Block2dHandleTemplate,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn create_handle(handle: Block2dHandleTemplate) -> Block2dMutation {
    Block2dMutation::CreateHandle(CreateHandle { handle })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for CreateHandle {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "handle", kind: "create-handle", record: "CreatedHandle" };

    async fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create handle \"{}\"", self.handle.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.handle.id.clone()]
    }
}
//#endregion 🔖️Mutation

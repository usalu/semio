//! 🌱️ Block2d mutation — `CreateHandleKind`: a new handle-kind catalog row.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::{Block2dHandleKind};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱️ `create-handle-kind` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
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

    fn diff(&self, base: &Block2dSnapshot) -> Block2dDiff {
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

//! 🌱 Block5d mutation — `CreateGripKind`: a new grip-kind catalog row.

use crate::artifacts::block5d::{Block5dGripKind, Block5dSnapshot};
use crate::artifacts::block5d::diff::{Block5dDiff, Block5dGripKindsDelta};
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Mutation
/// 🌱 `create-grip-kind` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-grip-kind")]
pub struct CreateGripKind {
    #[dsl(block)]
    pub grip_kind: Block5dGripKind,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_grip_kind(grip_kind: Block5dGripKind) -> Block5dMutation {
    Block5dMutation::CreateGripKind(CreateGripKind { grip_kind })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for CreateGripKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "grip-kind", kind: "create-grip-kind", record: "CreatedGripKind" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create grip kind \"{}\"", self.grip_kind.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.grip_kind.id.clone()]
    }
}
//#endregion 🔖️Mutation

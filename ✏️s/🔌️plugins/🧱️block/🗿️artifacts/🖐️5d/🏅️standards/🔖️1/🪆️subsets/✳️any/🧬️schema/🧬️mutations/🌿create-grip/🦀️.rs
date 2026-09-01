//! 🌿 Block5d mutation — `CreateGrip`: a new rim-grip template.

use crate::artifacts::block5d::{Block5dGripTemplate, Block5dSnapshot};
use crate::artifacts::block5d::diff::{Block5dDiff, Block5dGripsDelta};
use crate::artifacts::block5d::mutations::Block5dMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌿 `create-grip` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-grip")]
pub struct CreateGrip {
    #[dsl(block)]
    pub grip: Block5dGripTemplate,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn create_grip(grip: Block5dGripTemplate) -> Block5dMutation {
    Block5dMutation::CreateGrip(CreateGrip { grip })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for CreateGrip {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "grip", kind: "create-grip", record: "CreatedGrip" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create grip \"{}\"", self.grip.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.grip.id.clone()]
    }
}
//#endregion 🔖️Mutation

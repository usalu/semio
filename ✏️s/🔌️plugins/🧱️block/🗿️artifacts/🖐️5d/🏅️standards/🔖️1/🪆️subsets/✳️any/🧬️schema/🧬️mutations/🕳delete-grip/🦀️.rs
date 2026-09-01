//! 🕳 Block5d mutation — `DeleteGrip`: a rim-grip template.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::{Block5dDiff, Block5dGripsDelta};
use crate::artifacts::block5d::mutations::Block5dMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🕳 `delete-grip` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-grip")]
pub struct DeleteGrip {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn delete_grip(id: String) -> Block5dMutation {
    Block5dMutation::DeleteGrip(DeleteGrip { id })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for DeleteGrip {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "grip", kind: "delete-grip", record: "DeletedGrip" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete grip \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

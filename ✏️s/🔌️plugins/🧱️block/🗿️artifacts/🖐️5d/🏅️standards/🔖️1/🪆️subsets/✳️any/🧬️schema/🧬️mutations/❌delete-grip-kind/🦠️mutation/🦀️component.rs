//! ❌ Block5d mutation — `DeleteGripKind`: a grip-kind catalog row.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ❌ `delete-grip-kind` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-grip-kind")]
pub struct DeleteGripKind {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn delete_grip_kind(id: String) -> Block5dMutation {
    Block5dMutation::DeleteGripKind(DeleteGripKind { id })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for DeleteGripKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "grip-kind", kind: "delete-grip-kind", record: "DeletedGripKind" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete grip kind \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

//! 🗑️ TrinityGraph mutation — `DeleteNode`: removes an id-keyed node (captures cascade-severed
//! incident edges).
use crate::artifacts::jack::diff::JackDiff;
use crate::artifacts::jack::mutations::TrinityGraphMutation;
use crate::artifacts::jack::JackSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗑️ `delete-node` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteNode {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn delete_node(id: String) -> TrinityGraphMutation {
    TrinityGraphMutation::DeleteNode(DeleteNode { id })
}

impl protocol::MutationKind<JackSnapshot, TrinityGraphMutation> for DeleteNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "node", kind: "delete-node", record: "DeletedNode" };

    async fn diff(&self, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &JackSnapshot) -> Vec<TrinityGraphMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete node \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

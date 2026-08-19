//! 🗑️ DAG mutation — `DeleteNode`: removes an id-keyed node (captures cascade — any edge touching
//! this node is severed too, re-`connect-nodes`ed by the inverse).
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗑️ `delete-node` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteNode {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn delete_node(id: String) -> DagMutation {
    DagMutation::DeleteNode(DeleteNode { id })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for DeleteNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "node", kind: "delete-node", record: "DeletedNode" };

    async fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
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

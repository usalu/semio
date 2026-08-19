//! ✂️ TrinityGraph mutation — `DeleteEdge`: removes an id-keyed edge.
use crate::artifacts::jack::diff::JackDiff;
use crate::artifacts::jack::mutations::TrinityGraphMutation;
use crate::artifacts::jack::JackSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✂️ `delete-edge` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteEdge {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn delete_edge(id: String) -> TrinityGraphMutation {
    TrinityGraphMutation::DeleteEdge(DeleteEdge { id })
}

impl protocol::MutationKind<JackSnapshot, TrinityGraphMutation> for DeleteEdge {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "edge", kind: "delete-edge", record: "DeletedEdge" };

    async fn diff(&self, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &JackSnapshot) -> Vec<TrinityGraphMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete edge \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

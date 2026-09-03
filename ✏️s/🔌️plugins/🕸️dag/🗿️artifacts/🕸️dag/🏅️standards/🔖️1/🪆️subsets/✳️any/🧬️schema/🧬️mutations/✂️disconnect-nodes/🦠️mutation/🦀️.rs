//! ✂️ DAG mutation — `DisconnectNodes`: removes a port-to-port edge relationship.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Mutation
/// ✂️ `disconnect-nodes` payload — edge id.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DisconnectNodes {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn disconnect_nodes(id: String) -> DagMutation {
    DagMutation::DisconnectNodes(DisconnectNodes { id })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for DisconnectNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "nodes", kind: "disconnect-nodes", record: "DisconnectedNodes" };

    async fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Disconnect edge \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

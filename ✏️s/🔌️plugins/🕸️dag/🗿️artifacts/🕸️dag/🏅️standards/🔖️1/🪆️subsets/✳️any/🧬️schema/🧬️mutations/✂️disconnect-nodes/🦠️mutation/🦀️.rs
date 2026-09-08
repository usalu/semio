//! ✂️ DAG mutation — `DisconnectNodes`: removes a port-to-port edge relationship.
use crate::diff::DagDiff;
use crate::mutations::DagMutation;
use crate::DagSnapshot;

//#region 🔖️Mutation
/// ✂️ `disconnect-nodes` payload — edge id.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
#[derive(dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DisconnectNodes {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn disconnect_nodes(id: String) -> DagMutation {
    DagMutation::DisconnectNodes(DisconnectNodes { id })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for DisconnectNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "nodes", kind: "disconnect-nodes", record: "DisconnectedNodes" };

    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Disconnect edge \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

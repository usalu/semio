use super::super::{DisconnectEdge, WorkflowDiff, WorkflowEdge, WorkflowMutation, WorkflowSnapshot};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "connect-ports")]
pub struct ConnectPorts { pub edge: WorkflowEdge }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for ConnectPorts {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "workflow", kind: "connect-ports", record: "ConnectedWorkflowPorts" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> { protocol::MutationOutcome::new(WorkflowDiff::ConnectPorts { edge: self.edge.clone() }) }
    fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { vec![WorkflowMutation::DisconnectEdge(DisconnectEdge { edge_id: self.edge.id.clone() })] }
    fn label(&self) -> String { format!("Connect workflow ports {}", self.edge.id) }
    fn target(&self) -> Vec<String> { vec!["edges".into(), self.edge.id.clone()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_identity() { assert_eq!(<ConnectPorts as MutationLeaf>::DESCRIPTOR.semantic_kind, "connect-ports"); }
}

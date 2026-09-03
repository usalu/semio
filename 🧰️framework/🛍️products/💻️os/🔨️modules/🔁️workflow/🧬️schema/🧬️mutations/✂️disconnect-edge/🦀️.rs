use super::super::{ConnectPorts, WorkflowDiff, WorkflowMutation, WorkflowSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "disconnect-edge")]
pub struct DisconnectEdge { #[dsl(key = "id")] pub edge_id: String }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for DisconnectEdge {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "workflow", kind: "disconnect-edge", record: "DisconnectedWorkflowEdge" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> { protocol::MutationOutcome::new(WorkflowDiff::DisconnectEdge { edge_id: self.edge_id.clone() }) }
    fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { base.graph.edges.iter().find(|edge| edge.id == self.edge_id).map(|edge| vec![WorkflowMutation::ConnectPorts(ConnectPorts { edge: edge.clone() })]).unwrap_or_default() }
    fn label(&self) -> String { format!("Disconnect workflow edge {}", self.edge_id) }
    fn target(&self) -> Vec<String> { vec!["edges".into(), self.edge_id.clone()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_identity() { assert_eq!(<DisconnectEdge as MutationLeaf>::DESCRIPTOR.semantic_kind, "disconnect-edge"); }
}

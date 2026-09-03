use super::super::{AddNode, BindInput, BindOutput, BindParameterField, ConnectPorts, WorkflowDiff, WorkflowMutation, WorkflowSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "remove-node")]
pub struct RemoveNode { #[dsl(key = "id")] pub node_id: String }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for RemoveNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "workflow", kind: "remove-node", record: "RemovedWorkflowNode" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> { protocol::MutationOutcome::new(WorkflowDiff::RemoveNode { node_id: self.node_id.clone() }) }
    fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { { let Some(node) = base.graph.nodes.iter().find(|node| node.id == self.node_id) else { return Vec::new() }; let mut ops: Vec<WorkflowMutation> = base.graph.edges.iter().filter(|edge| edge.source_node_id == self.node_id || edge.target_node_id == self.node_id).map(|edge| WorkflowMutation::ConnectPorts(ConnectPorts { edge: edge.clone() })).collect(); ops.extend(base.parameter_bindings.iter().filter(|binding| binding.node_id == self.node_id).map(|binding| WorkflowMutation::BindParameterField(BindParameterField { binding: binding.clone() }))); ops.extend(base.input_bindings.iter().filter(|binding| binding.node_id == self.node_id).map(|binding| WorkflowMutation::BindInput(BindInput { binding: binding.clone() }))); ops.extend(base.output_bindings.iter().filter(|binding| binding.node_id == self.node_id).map(|binding| WorkflowMutation::BindOutput(BindOutput { binding: binding.clone() }))); ops.push(WorkflowMutation::AddNode(AddNode { node: node.clone() })); ops } }
    fn label(&self) -> String { format!("Remove workflow node {}", self.node_id) }
    fn target(&self) -> Vec<String> { vec!["nodes".into(), self.node_id.clone()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_identity() { assert_eq!(<RemoveNode as MutationLeaf>::DESCRIPTOR.semantic_kind, "remove-node"); }
}

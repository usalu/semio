use super::super::{RemoveNode, WorkflowDiff, WorkflowMutation, WorkflowNode, WorkflowSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "add-node")]
pub struct AddNode { pub node: WorkflowNode }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for AddNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "workflow", kind: "add-node", record: "AddedWorkflowNode" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> { protocol::MutationOutcome::new(WorkflowDiff::AddNode { node: self.node.clone() }) }
    fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { vec![WorkflowMutation::RemoveNode(RemoveNode { node_id: self.node.id.clone() })] }
    fn label(&self) -> String { format!("Add workflow node {}", self.node.id) }
    fn target(&self) -> Vec<String> { vec!["nodes".into(), self.node.id.clone()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_identity() { assert_eq!(<AddNode as MutationLeaf>::DESCRIPTOR.semantic_kind, "add-node"); }
}

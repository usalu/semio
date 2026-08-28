use super::super::{WorkflowDiff, WorkflowMutation, WorkflowSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "rename-node")]
pub struct RenameNode { #[dsl(key = "id")] pub node_id: String, pub label: String }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for RenameNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "workflow", kind: "rename-node", record: "RenamedWorkflowNode" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> { protocol::MutationOutcome::new(WorkflowDiff::PatchNode { node_id: self.node_id.clone(), label: self.label.clone() }) }
    fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { base.graph.nodes.iter().find(|node| node.id == self.node_id).map(|node| vec![WorkflowMutation::RenameNode(RenameNode { node_id: self.node_id.clone(), label: node.label.clone() })]).unwrap_or_default() }
    fn label(&self) -> String { format!("Rename workflow node {}", self.node_id) }
    fn target(&self) -> Vec<String> { vec!["nodes".into(), self.node_id.clone()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_identity() { assert_eq!(<RenameNode as MutationLeaf>::DESCRIPTOR.semantic_kind, "rename-node"); }
}

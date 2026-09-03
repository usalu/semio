use super::super::{WorkflowDiff, WorkflowMutation, WorkflowSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "move-node")]
pub struct MoveNode { #[dsl(key = "id")] pub node_id: String, pub x: f64, pub y: f64 }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for MoveNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "workflow", kind: "move-node", record: "MovedWorkflowNode" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> { protocol::MutationOutcome::new(WorkflowDiff::MoveNode { node_id: self.node_id.clone(), x: self.x, y: self.y }) }
    fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { base.graph.nodes.iter().find(|node| node.id == self.node_id).map(|node| vec![WorkflowMutation::MoveNode(MoveNode { node_id: self.node_id.clone(), x: node.x, y: node.y })]).unwrap_or_default() }
    fn label(&self) -> String { format!("Move workflow node {}", self.node_id) }
    fn target(&self) -> Vec<String> { vec!["nodes".into(), self.node_id.clone()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_identity() { assert_eq!(<MoveNode as MutationLeaf>::DESCRIPTOR.semantic_kind, "move-node"); }
}

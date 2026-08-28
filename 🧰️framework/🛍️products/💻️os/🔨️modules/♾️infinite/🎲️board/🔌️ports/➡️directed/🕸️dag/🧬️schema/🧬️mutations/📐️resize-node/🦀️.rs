use super::super::{DagDelta, DagDiff, DagMutation, DagSnapshot, ResizedNode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "resize-node")]
pub struct ResizeNode { pub id: String, pub width: f64, pub height: f64 }

impl protocol::MutationKind<DagSnapshot, DagMutation> for ResizeNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "resize", entity: "node", kind: "resize-node", record: "ResizedNode" };
    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        if !base.nodes.iter().any(|node| node.id == self.id) { return protocol::MutationOutcome::new(DagDiff::default()); }
        protocol::MutationOutcome::new(DagDiff::from(DagDelta { resized_node: Some(ResizedNode { id: self.id.clone(), width: self.width, height: self.height }), ..Default::default() }))
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        base.nodes.iter().find(|node| node.id == self.id).map(|node| vec![DagMutation::ResizeNode(Self { id: self.id.clone(), width: node.width, height: node.height })]).unwrap_or_default()
    }
    fn label(&self) -> String { format!("Resize node {}", self.id) }
    fn target(&self) -> Vec<String> { vec!["nodes".into(), self.id.clone()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::dag_direct_tests::assert_leaf_contract::<ResizeNode>(5, DagMutation::ResizeNode, include_str!("🔣️.json")); }
}

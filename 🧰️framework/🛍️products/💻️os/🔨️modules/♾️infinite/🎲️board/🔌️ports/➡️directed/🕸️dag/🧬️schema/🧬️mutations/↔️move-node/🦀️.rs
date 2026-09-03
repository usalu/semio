use super::super::{DagDelta, DagDiff, DagMutation, DagSnapshot, MovedNode};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "move-node")]
pub struct MoveNode { pub id: String, pub x: f64, pub y: f64 }

impl protocol::MutationKind<DagSnapshot, DagMutation> for MoveNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "node", kind: "move-node", record: "MovedNode" };
    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        if !base.nodes.iter().any(|node| node.id == self.id) { return protocol::MutationOutcome::new(DagDiff::default()); }
        protocol::MutationOutcome::new(DagDiff::from(DagDelta { moved_node: Some(MovedNode { id: self.id.clone(), x: self.x, y: self.y }), ..Default::default() }))
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        base.nodes.iter().find(|node| node.id == self.id).map(|node| vec![DagMutation::MoveNode(Self { id: self.id.clone(), x: node.x, y: node.y })]).unwrap_or_default()
    }
    fn label(&self) -> String { format!("Move node {}", self.id) }
    fn target(&self) -> Vec<String> { vec!["nodes".into(), self.id.clone()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::dag_direct_tests::assert_leaf_contract::<MoveNode>(4, DagMutation::MoveNode, include_str!("🔣️.json")); }
}

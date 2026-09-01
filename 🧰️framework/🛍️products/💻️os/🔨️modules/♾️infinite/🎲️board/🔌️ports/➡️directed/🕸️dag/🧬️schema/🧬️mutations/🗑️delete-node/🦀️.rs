use super::super::{DagDelta, DagDiff, DagMutation, DagSnapshot, CreateNode, ConnectNodes, dag_index_to_wire, split_dag_endpoint};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "delete-node")]
pub struct DeleteNode { pub id: String }

impl protocol::MutationKind<DagSnapshot, DagMutation> for DeleteNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "node", kind: "delete-node", record: "DeletedNode" };
    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        if !base.nodes.iter().any(|node| node.id == self.id) { return protocol::MutationOutcome::new(DagDiff::default()); }
        let ids: Vec<String> = base.edges.iter().filter(|edge| split_dag_endpoint(&edge.source).0 == self.id || split_dag_endpoint(&edge.target).0 == self.id).map(|edge| edge.id.clone()).collect();
        protocol::MutationOutcome::new(DagDiff::from(DagDelta { deleted_node_ids: Some(vec![self.id.clone()]), disconnected_edge_ids: (!ids.is_empty()).then_some(ids), ..Default::default() }))
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        let Some(index) = base.nodes.iter().position(|node| node.id == self.id) else { return Vec::new(); };
        let mut inverse: Vec<DagMutation> = base.edges.iter().enumerate().rev().filter(|(_, edge)| split_dag_endpoint(&edge.source).0 == self.id || split_dag_endpoint(&edge.target).0 == self.id).map(|(index, edge)| DagMutation::ConnectNodes(ConnectNodes { id: edge.id.clone(), source: edge.source.clone(), target: edge.target.clone(), route_style: edge.route_style, properties: edge.properties.clone(), index: dag_index_to_wire(index) })).collect();
        inverse.push(DagMutation::CreateNode(CreateNode { node: base.nodes[index].clone(), index: dag_index_to_wire(index) }));
        inverse
    }
    fn label(&self) -> String { format!("Delete node {}", self.id) }
    fn target(&self) -> Vec<String> { vec!["nodes".into(), self.id.clone()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::dag_direct_tests::assert_leaf_contract::<DeleteNode>(1, DagMutation::DeleteNode, include_str!("🔣️.json")); }
}

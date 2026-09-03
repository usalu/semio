use super::super::{DagDelta, DagDiff, DagMutation, DagSnapshot, ConnectNodes, dag_index_to_wire};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "disconnect-nodes")]
pub struct DisconnectNodes { pub id: String }

impl protocol::MutationKind<DagSnapshot, DagMutation> for DisconnectNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "nodes", kind: "disconnect-nodes", record: "DisconnectedNodes" };
    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        if !base.edges.iter().any(|edge| edge.id == self.id) { return protocol::MutationOutcome::new(DagDiff::default()); }
        protocol::MutationOutcome::new(DagDiff::from(DagDelta { disconnected_edge_ids: Some(vec![self.id.clone()]), ..Default::default() }))
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        base.edges.iter().enumerate().find(|(_, edge)| edge.id == self.id).map(|(index, edge)| vec![DagMutation::ConnectNodes(ConnectNodes { id: edge.id.clone(), source: edge.source.clone(), target: edge.target.clone(), route_style: edge.route_style, properties: edge.properties.clone(), index: dag_index_to_wire(index) })]).unwrap_or_default()
    }
    fn label(&self) -> String { format!("Disconnect {}", self.id) }
    fn target(&self) -> Vec<String> { vec!["edges".into(), self.id.clone()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::dag_direct_tests::assert_leaf_contract::<DisconnectNodes>(13, DagMutation::DisconnectNodes, include_str!("🔣️.json")); }
}

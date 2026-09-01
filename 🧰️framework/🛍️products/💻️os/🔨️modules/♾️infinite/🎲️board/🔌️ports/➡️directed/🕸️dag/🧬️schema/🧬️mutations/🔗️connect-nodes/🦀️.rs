use super::super::{DagDelta, DagDiff, DagMutation, DagSnapshot, DagFixtureEdge, EdgeRouteStyle, PropertyBag, DisconnectNodes};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "connect-nodes")]
pub struct ConnectNodes { pub id: String, pub source: String, pub target: String, pub route_style: EdgeRouteStyle, pub properties: PropertyBag, pub index: u64 }

impl protocol::MutationKind<DagSnapshot, DagMutation> for ConnectNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "nodes", kind: "connect-nodes", record: "ConnectedNodes" };
    fn diff(&self, _base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        protocol::MutationOutcome::new(DagDiff::from(DagDelta { connected_edge: Some(DagFixtureEdge { id: self.id.clone(), source: self.source.clone(), target: self.target.clone(), route_style: self.route_style, properties: self.properties.clone() }), connected_edge_at: Some(self.index), ..Default::default() }))
    }
    fn inverse(&self, _base: &DagSnapshot) -> Vec<DagMutation> {
        vec![DagMutation::DisconnectNodes(DisconnectNodes { id: self.id.clone() })]
    }
    fn label(&self) -> String { format!("Connect {}", self.id) }
    fn target(&self) -> Vec<String> { vec!["edges".into(), self.id.clone()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::dag_direct_tests::assert_leaf_contract::<ConnectNodes>(12, DagMutation::ConnectNodes, include_str!("🔣️.json")); }
}

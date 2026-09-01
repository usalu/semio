use super::super::{DagDelta, DagDiff, DagMutation, DagSnapshot, RenamedNode, RewrittenEdgeEndpoint, split_dag_endpoint};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "rename-node")]
pub struct RenameNode { pub id: String, pub new_id: String }

impl protocol::MutationKind<DagSnapshot, DagMutation> for RenameNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "node", kind: "rename-node", record: "RenamedNode" };
    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        if !base.nodes.iter().any(|node| node.id == self.id) { return protocol::MutationOutcome::new(DagDiff::default()); }
        let rewrites: Vec<RewrittenEdgeEndpoint> = base.edges.iter().filter_map(|edge| {
            let source = split_dag_endpoint(&edge.source).0;
            let target = split_dag_endpoint(&edge.target).0;
            (source == self.id || target == self.id).then(|| RewrittenEdgeEndpoint { id: edge.id.clone(), new_source: (source == self.id).then(|| format!("{}{}", self.new_id, &edge.source[self.id.len()..])), new_target: (target == self.id).then(|| format!("{}{}", self.new_id, &edge.target[self.id.len()..])) })
        }).collect();
        protocol::MutationOutcome::new(DagDiff::from(DagDelta { renamed_node: Some(RenamedNode { id: self.id.clone(), new_id: self.new_id.clone() }), rewritten_edge_endpoints: (!rewrites.is_empty()).then_some(rewrites), ..Default::default() }))
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        base.nodes.iter().find(|node| node.id == self.id).map(|_| vec![DagMutation::RenameNode(Self { id: self.new_id.clone(), new_id: self.id.clone() })]).unwrap_or_default()
    }
    fn label(&self) -> String { format!("Rename node {}", self.id) }
    fn target(&self) -> Vec<String> { vec!["nodes".into(), self.id.clone()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::dag_direct_tests::assert_leaf_contract::<RenameNode>(2, DagMutation::RenameNode, include_str!("🔣️.json")); }
}

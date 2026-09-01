use super::super::{DagDelta, DagDiff, DagMutation, DagSnapshot, DagNodeSpec, DeleteNode};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "create-node")]
pub struct CreateNode { pub node: DagNodeSpec, pub index: u64 }

impl protocol::MutationKind<DagSnapshot, DagMutation> for CreateNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "node", kind: "create-node", record: "CreatedNode" };
    fn diff(&self, _base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        protocol::MutationOutcome::new(DagDiff::from(DagDelta { created_node: Some(self.node.clone()), created_node_at: Some(self.index), ..Default::default() }))
    }
    fn inverse(&self, _base: &DagSnapshot) -> Vec<DagMutation> {
        vec![DagMutation::DeleteNode(DeleteNode { id: self.node.id.clone() })]
    }
    fn label(&self) -> String { format!("Create node {}", self.node.id) }
    fn target(&self) -> Vec<String> { vec!["nodes".into(), self.node.id.clone()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::dag_direct_tests::assert_leaf_contract::<CreateNode>(0, DagMutation::CreateNode, include_str!("🔣️.json")); }
}

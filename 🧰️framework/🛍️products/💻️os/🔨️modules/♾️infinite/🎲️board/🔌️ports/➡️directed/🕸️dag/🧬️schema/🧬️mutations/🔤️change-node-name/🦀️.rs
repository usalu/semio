use super::super::{DagDelta, DagDiff, DagMutation, DagSnapshot, ChangedNodeName};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "change-node-name")]
pub struct ChangeNodeName { pub id: String, pub new_name: String }

impl protocol::MutationKind<DagSnapshot, DagMutation> for ChangeNodeName {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-name", kind: "change-node-name", record: "ChangedNodeName" };
    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        if !base.nodes.iter().any(|node| node.id == self.id) { return protocol::MutationOutcome::new(DagDiff::default()); }
        protocol::MutationOutcome::new(DagDiff::from(DagDelta { changed_node_name: Some(ChangedNodeName { id: self.id.clone(), new_name: self.new_name.clone() }), ..Default::default() }))
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        base.nodes.iter().find(|node| node.id == self.id).map(|node| vec![DagMutation::ChangeNodeName(Self { id: self.id.clone(), new_name: node.name.clone() })]).unwrap_or_default()
    }
    fn label(&self) -> String { format!("Change node name {}", self.id) }
    fn target(&self) -> Vec<String> { vec!["nodes".into(), self.id.clone()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::dag_direct_tests::assert_leaf_contract::<ChangeNodeName>(3, DagMutation::ChangeNodeName, include_str!("🔣️.json")); }
}

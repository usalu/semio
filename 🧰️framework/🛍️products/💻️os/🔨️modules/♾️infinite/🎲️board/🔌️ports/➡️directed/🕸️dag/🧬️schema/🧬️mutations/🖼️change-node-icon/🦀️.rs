use super::super::{DagDelta, DagDiff, DagMutation, DagSnapshot, ChangedNodeIcon};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "change-node-icon")]
pub struct ChangeNodeIcon { pub id: String, pub new_icon: String }

impl protocol::MutationKind<DagSnapshot, DagMutation> for ChangeNodeIcon {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-icon", kind: "change-node-icon", record: "ChangedNodeIcon" };
    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        if !base.nodes.iter().any(|node| node.id == self.id) { return protocol::MutationOutcome::new(DagDiff::default()); }
        protocol::MutationOutcome::new(DagDiff::from(DagDelta { changed_node_icon: Some(ChangedNodeIcon { id: self.id.clone(), new_icon: self.new_icon.clone() }), ..Default::default() }))
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        base.nodes.iter().find(|node| node.id == self.id).map(|node| vec![DagMutation::ChangeNodeIcon(Self { id: self.id.clone(), new_icon: node.icon.clone() })]).unwrap_or_default()
    }
    fn label(&self) -> String { format!("Change node icon {}", self.id) }
    fn target(&self) -> Vec<String> { vec!["nodes".into(), self.id.clone()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::dag_direct_tests::assert_leaf_contract::<ChangeNodeIcon>(6, DagMutation::ChangeNodeIcon, include_str!("🔣️.json")); }
}

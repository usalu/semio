use super::super::{DagDelta, DagDiff, DagMutation, DagSnapshot, ChangedNodeAbbreviation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "change-node-abbreviation")]
pub struct ChangeNodeAbbreviation { pub id: String, pub new_abbreviation: String }

impl protocol::MutationKind<DagSnapshot, DagMutation> for ChangeNodeAbbreviation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-abbreviation", kind: "change-node-abbreviation", record: "ChangedNodeAbbreviation" };
    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        if !base.nodes.iter().any(|node| node.id == self.id) { return protocol::MutationOutcome::new(DagDiff::default()); }
        protocol::MutationOutcome::new(DagDiff::from(DagDelta { changed_node_abbreviation: Some(ChangedNodeAbbreviation { id: self.id.clone(), new_abbreviation: self.new_abbreviation.clone() }), ..Default::default() }))
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        base.nodes.iter().find(|node| node.id == self.id).map(|node| vec![DagMutation::ChangeNodeAbbreviation(Self { id: self.id.clone(), new_abbreviation: node.abbreviation.clone() })]).unwrap_or_default()
    }
    fn label(&self) -> String { format!("Change node abbreviation {}", self.id) }
    fn target(&self) -> Vec<String> { vec!["nodes".into(), self.id.clone()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::dag_direct_tests::assert_leaf_contract::<ChangeNodeAbbreviation>(7, DagMutation::ChangeNodeAbbreviation, include_str!("🔣️.json")); }
}

use super::super::{DagDelta, DagDiff, DagMutation, DagSnapshot, ChangedNodeOperatorKind};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "change-node-operator-kind")]
pub struct ChangeNodeOperatorKind { pub id: String, pub new_operator_kind: Option<String> }

impl protocol::MutationKind<DagSnapshot, DagMutation> for ChangeNodeOperatorKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-operator-kind", kind: "change-node-operator-kind", record: "ChangedNodeOperatorKind" };
    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        if !base.nodes.iter().any(|node| node.id == self.id) { return protocol::MutationOutcome::new(DagDiff::default()); }
        protocol::MutationOutcome::new(DagDiff::from(DagDelta { changed_node_operator_kind: Some(ChangedNodeOperatorKind { id: self.id.clone(), new_operator_kind: self.new_operator_kind.clone() }), ..Default::default() }))
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        base.nodes.iter().find(|node| node.id == self.id).map(|node| vec![DagMutation::ChangeNodeOperatorKind(Self { id: self.id.clone(), new_operator_kind: node.operator_kind.clone() })]).unwrap_or_default()
    }
    fn label(&self) -> String { format!("Change node operator kind {}", self.id) }
    fn target(&self) -> Vec<String> { vec!["nodes".into(), self.id.clone()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::dag_direct_tests::assert_leaf_contract::<ChangeNodeOperatorKind>(8, DagMutation::ChangeNodeOperatorKind, include_str!("🔣️.json")); }
}

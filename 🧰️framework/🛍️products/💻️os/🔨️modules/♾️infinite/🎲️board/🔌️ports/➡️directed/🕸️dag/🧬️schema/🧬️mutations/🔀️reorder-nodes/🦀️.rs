use super::super::{DagDelta, DagDiff, DagMutation, DagSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "reorder-nodes")]
pub struct ReorderNodes { pub order: Vec<String> }

impl protocol::MutationKind<DagSnapshot, DagMutation> for ReorderNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "nodes", kind: "reorder-nodes", record: "ReorderedNodes" };
    fn diff(&self, _base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        protocol::MutationOutcome::new(DagDiff::from(DagDelta { reordered_nodes: Some(self.order.clone()), ..Default::default() }))
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        vec![DagMutation::ReorderNodes(Self { order: base.nodes.iter().map(|node| node.id.clone()).collect() })]
    }
    fn label(&self) -> String { "Reorder nodes".into() }
    fn target(&self) -> Vec<String> { vec!["nodes".into()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::dag_direct_tests::assert_leaf_contract::<ReorderNodes>(11, DagMutation::ReorderNodes, include_str!("🔣️.json")); }
}

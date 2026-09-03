use super::super::{DagDelta, DagDiff, DagMutation, DagSnapshot, ReplacedNodeProperties, PropertyBag};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-node-properties")]
pub struct ReplaceNodeProperties { pub id: String, pub new_properties: PropertyBag }

impl protocol::MutationKind<DagSnapshot, DagMutation> for ReplaceNodeProperties {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "node-properties", kind: "replace-node-properties", record: "ReplacedNodeProperties" };
    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        if !base.nodes.iter().any(|node| node.id == self.id) { return protocol::MutationOutcome::new(DagDiff::default()); }
        protocol::MutationOutcome::new(DagDiff::from(DagDelta { replaced_node_properties: Some(ReplacedNodeProperties { id: self.id.clone(), new_properties: self.new_properties.clone() }), ..Default::default() }))
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        base.nodes.iter().find(|node| node.id == self.id).map(|node| vec![DagMutation::ReplaceNodeProperties(Self { id: self.id.clone(), new_properties: node.properties.clone() })]).unwrap_or_default()
    }
    fn label(&self) -> String { format!("Replace node properties {}", self.id) }
    fn target(&self) -> Vec<String> { vec!["nodes".into(), self.id.clone()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::dag_direct_tests::assert_leaf_contract::<ReplaceNodeProperties>(10, DagMutation::ReplaceNodeProperties, include_str!("🔣️.json")); }
}

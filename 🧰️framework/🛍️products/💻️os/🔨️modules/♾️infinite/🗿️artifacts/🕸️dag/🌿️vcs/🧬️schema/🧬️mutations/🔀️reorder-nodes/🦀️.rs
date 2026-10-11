use super::super::{DagDelta, DagDiff, DagMutation, DagSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "reorder-nodes")]
pub struct ReorderNodes {
    pub order: Vec<String>,
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for ReorderNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "nodes", kind: "reorder-nodes", record: "ReorderedNodes" };
    fn diff(&self, _base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        protocol::MutationOutcome::new(DagDiff::from(DagDelta { reordered_nodes: Some(self.order.clone()), ..Default::default() }))
    }
    fn inverse(&self, base: &DagSnapshot) -> Result<Vec<DagMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![DagMutation::ReorderNodes(Self { order: base.nodes.iter().map(|node| node.id.clone()).collect() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Reorder nodes", "Knoten umordnen")
    }
    fn target(&self) -> Vec<String> {
        vec!["nodes".into()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

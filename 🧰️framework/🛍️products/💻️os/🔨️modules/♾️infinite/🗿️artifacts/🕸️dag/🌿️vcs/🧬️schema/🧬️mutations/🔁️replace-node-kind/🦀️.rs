use super::super::{DagDelta, DagDiff, DagMutation, DagNodeKind, DagSnapshot, ReplacedNodeKind};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-node-kind")]
pub struct ReplaceNodeKind {
    pub id: String,
    pub new_kind: DagNodeKind,
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for ReplaceNodeKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "node-kind", kind: "replace-node-kind", record: "ReplacedNodeKind" };
    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        if !base.nodes.iter().any(|node| node.id == self.id) {
            return protocol::MutationOutcome::new(DagDiff::default());
        }
        protocol::MutationOutcome::new(DagDiff::from(DagDelta { replaced_node_kind: Some(ReplacedNodeKind { id: self.id.clone(), new_kind: self.new_kind.clone() }), ..Default::default() }))
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        base.nodes.iter().find(|node| node.id == self.id).map(|node| vec![DagMutation::ReplaceNodeKind(Self { id: self.id.clone(), new_kind: node.kind.clone() })]).unwrap_or_default()
    }
    fn label(&self) -> String {
        format!("Replace node kind {}", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec!["nodes".into(), self.id.clone()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//! 📐 DAG mutation — `ResizeNode`: absolute extent change of a canvas node.
use crate::diff::DagDiff;
use crate::mutations::DagMutation;
use crate::DagSnapshot;

//#region 🔖️Mutation
/// 📐 `resize-node` payload — FINAL-state absolute `(width, height)`.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
#[derive(dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ResizeNode {
    pub id: String,
    pub width: f64,
    pub height: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_node(id: String, width: f64, height: f64) -> DagMutation {
    DagMutation::ResizeNode(ResizeNode { id, width, height })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for ResizeNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "resize", entity: "node", kind: "resize-node", record: "ResizedNode" };

    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Resize node \"{}\" to ({}, {})", self.id, self.width, self.height)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

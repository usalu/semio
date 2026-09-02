//! 📐 DAG mutation — `ResizeNode`: absolute extent change of a canvas node.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📐 `resize-node` payload — FINAL-state absolute `(width, height)`.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, Serialize, Deserialize)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct ResizeNode {
    pub id: String,
    pub width: f64,
    pub height: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn resize_node(id: String, width: f64, height: f64) -> DagMutation {
    DagMutation::ResizeNode(ResizeNode { id, width, height })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for ResizeNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "resize", entity: "node", kind: "resize-node", record: "ResizedNode" };

    async fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Resize node \"{}\" to ({}, {})", self.id, self.width, self.height)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

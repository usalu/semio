//! ↔️ DAG mutation — `MoveNode`: absolute spatial reposition of a canvas node.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ↔️ `move-node` payload — FINAL-state absolute `(x, y)`.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, Serialize, Deserialize)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct MoveNode {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn move_node(id: String, x: f64, y: f64) -> DagMutation {
    DagMutation::MoveNode(MoveNode { id, x, y })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for MoveNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "node", kind: "move-node", record: "MovedNode" };

    async fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Move node \"{}\" to ({}, {})", self.id, self.x, self.y)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

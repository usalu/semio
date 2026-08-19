//! 📍️ TrinityGraph mutation — `MoveNode`: absolute spatial reposition of a node.
use crate::artifacts::jack::diff::JackDiff;
use crate::artifacts::jack::mutations::TrinityGraphMutation;
use crate::artifacts::jack::JackSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📍️ `move-node` payload — FINAL-state absolute `(x, y)`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveNode {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn move_node(id: String, x: f64, y: f64) -> TrinityGraphMutation {
    TrinityGraphMutation::MoveNode(MoveNode { id, x, y })
}

impl protocol::MutationKind<JackSnapshot, TrinityGraphMutation> for MoveNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "node", kind: "move-node", record: "MovedNode" };

    async fn diff(&self, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &JackSnapshot) -> Vec<TrinityGraphMutation> {
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

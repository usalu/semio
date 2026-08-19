//! 🏗️ Puzzle2d mutation — `ChangeNodeKind`: changes a node's `node_kind` catalog reference.
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🏗️ `change-node-kind` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-node-kind")]
pub struct ChangeNodeKind {
    pub id: String,
    pub new_node_kind: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_node_kind(id: String, new_node_kind: Option<String>) -> Puzzle2dMutation {
    Puzzle2dMutation::ChangeNodeKind(ChangeNodeKind { id, new_node_kind })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ChangeNodeKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node", kind: "change-node-kind", record: "ChangedNodeKind" };

    async fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change node \"{}\" kind", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

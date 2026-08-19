//! Puzzle2d mutation — `ChangeEdgeLocked`: changes an edge's locked flag.
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `change-edge-locked` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-edge-locked")]
pub struct ChangeEdgeLocked {
    pub id: String,
    pub new_locked: Option<bool>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_edge_locked(id: String, new_locked: Option<bool>) -> Puzzle2dMutation {
    Puzzle2dMutation::ChangeEdgeLocked(ChangeEdgeLocked { id, new_locked })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ChangeEdgeLocked {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "edge", kind: "change-edge-locked", record: "ChangedEdgeLocked" };

    async fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change edge \"{}\" locked", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

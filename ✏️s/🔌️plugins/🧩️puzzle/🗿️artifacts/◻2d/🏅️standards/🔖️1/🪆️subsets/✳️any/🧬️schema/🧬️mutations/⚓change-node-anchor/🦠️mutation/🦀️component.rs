//! ⚓️ Puzzle2d mutation — `ChangeNodeAnchor`: changes whether a node keeps its stored pose or derives it from edges.
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⚓️ `change-node-anchor` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-node-anchor")]
pub struct ChangeNodeAnchor {
    pub id: String,
    pub new_anchor: crate::artifacts::puzzle2d::Puzzle2dNodeAnchor,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_node_anchor(id: String, new_anchor: crate::artifacts::puzzle2d::Puzzle2dNodeAnchor) -> Puzzle2dMutation {
    Puzzle2dMutation::ChangeNodeAnchor(ChangeNodeAnchor { id, new_anchor })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ChangeNodeAnchor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node", kind: "change-node-anchor", record: "ChangedNodeAnchor" };

    async fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change node \"{}\" anchor", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

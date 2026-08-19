//! 👁️ Puzzle2d mutation — `ChangeNodeVisible`: changes a node's visibility flag.
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 👁️ `change-node-visible` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-node-visible")]
pub struct ChangeNodeVisible {
    pub id: String,
    pub new_visible: Option<bool>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_node_visible(id: String, new_visible: Option<bool>) -> Puzzle2dMutation {
    Puzzle2dMutation::ChangeNodeVisible(ChangeNodeVisible { id, new_visible })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ChangeNodeVisible {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node", kind: "change-node-visible", record: "ChangedNodeVisible" };

    async fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change node \"{}\" visible", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

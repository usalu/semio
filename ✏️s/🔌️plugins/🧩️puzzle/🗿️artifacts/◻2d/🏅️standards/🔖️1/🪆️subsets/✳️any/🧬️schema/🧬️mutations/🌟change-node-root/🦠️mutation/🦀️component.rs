//! 🌟️ Puzzle2d mutation — `ChangeNodeRoot`: changes a node's root/seed flag.
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌟️ `change-node-root` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-node-root")]
pub struct ChangeNodeRoot {
    pub id: String,
    pub new_root: Option<bool>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_node_root(id: String, new_root: Option<bool>) -> Puzzle2dMutation {
    Puzzle2dMutation::ChangeNodeRoot(ChangeNodeRoot { id, new_root })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ChangeNodeRoot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node", kind: "change-node-root", record: "ChangedNodeRoot" };

    async fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change node \"{}\" root", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

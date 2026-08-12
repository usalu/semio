//! ✏️️ Puzzle2d mutation — `EditNodeText`: replaces a node's authored display text.
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✏️️ `edit-node-text` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "edit-node-text")]
pub struct EditNodeText {
    pub id: String,
    pub new_text: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_node_text(id: String, new_text: Option<String>) -> Puzzle2dMutation {
    Puzzle2dMutation::EditNodeText(EditNodeText { id, new_text })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for EditNodeText {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "node", kind: "edit-node-text", record: "EditedNodeText" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> Puzzle2dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Edit node \"{}\" text", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

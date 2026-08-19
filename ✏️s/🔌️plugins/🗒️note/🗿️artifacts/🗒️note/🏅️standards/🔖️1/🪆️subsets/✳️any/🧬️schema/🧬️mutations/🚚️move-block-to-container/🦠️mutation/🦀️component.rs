//! 🚚 Note mutation — `MoveBlockToContainer`: reparents a block into a new container at an index (hierarchy move).
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🚚 `move-block-to-container` payload — reparents a block into a new container at an index (hierarchy move).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "move-block-to-container")]
pub struct MoveBlockToContainer {
    pub id: String,
    pub new_parent_id: Option<String>,
    pub index: usize,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn move_block_to_container(id: String, new_parent_id: Option<String>, index: usize) -> NoteMutation {
    NoteMutation::MoveBlockToContainer(MoveBlockToContainer { id, new_parent_id, index })
}

impl MutationKind<NoteSnapshot, NoteMutation> for MoveBlockToContainer {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "block", kind: "move-block-to-container", record: "MovedBlockToContainer" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Move block \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

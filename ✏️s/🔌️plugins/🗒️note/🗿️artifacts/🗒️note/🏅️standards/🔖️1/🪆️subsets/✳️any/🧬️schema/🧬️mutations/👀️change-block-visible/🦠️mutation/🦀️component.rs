//! 👀 Note mutation — `ChangeBlockVisible`: sets a block's visibility.
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 👀 `change-block-visible` payload — sets a block's visibility.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-block-visible")]
pub struct ChangeBlockVisible {
    pub id: String,
    pub new_visible: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_block_visible(id: String, new_visible: bool) -> NoteMutation {
    NoteMutation::ChangeBlockVisible(ChangeBlockVisible { id, new_visible })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ChangeBlockVisible {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "block-visible", kind: "change-block-visible", record: "ChangedBlockVisible" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change block \"{}\" visible to {}", self.id, self.new_visible)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

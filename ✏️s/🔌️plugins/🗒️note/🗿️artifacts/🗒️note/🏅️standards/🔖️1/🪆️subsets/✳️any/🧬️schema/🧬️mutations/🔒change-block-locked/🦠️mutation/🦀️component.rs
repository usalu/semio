//! 🔒 Note mutation — `ChangeBlockLocked`: sets a block's locked state.
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔒 `change-block-locked` payload — sets a block's locked state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-block-locked")]
pub struct ChangeBlockLocked {
    pub id: String,
    pub new_locked: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_block_locked(id: String, new_locked: bool) -> NoteMutation {
    NoteMutation::ChangeBlockLocked(ChangeBlockLocked { id, new_locked })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ChangeBlockLocked {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "block-locked", kind: "change-block-locked", record: "ChangedBlockLocked" };

    fn diff(&self, base: &NoteSnapshot) -> NoteDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change block \"{}\" locked to {}", self.id, self.new_locked)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

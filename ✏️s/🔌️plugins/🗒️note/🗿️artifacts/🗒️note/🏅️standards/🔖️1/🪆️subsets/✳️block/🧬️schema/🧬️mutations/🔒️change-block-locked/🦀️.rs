//! 🔒 Note mutation — `ChangeBlockLocked`: sets a block's locked state.

use crate::artifacts::note::{NoteDiff, NoteSnapshot};
use crate::artifacts::note::schema::diff::note_block_patch_diff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔒 `change-block-locked` payload — sets a block's locked state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-block-locked")]
pub struct ChangeBlockLocked {
    pub id: String,
    pub new_locked: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_block_locked(id: String, new_locked: bool) -> NoteMutation {
    NoteMutation::ChangeBlockLocked(ChangeBlockLocked { id, new_locked })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ChangeBlockLocked {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "block-locked", kind: "change-block-locked", record: "ChangedBlockLocked" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change block \"{}\" locked to {}", self.id, self.new_locked)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

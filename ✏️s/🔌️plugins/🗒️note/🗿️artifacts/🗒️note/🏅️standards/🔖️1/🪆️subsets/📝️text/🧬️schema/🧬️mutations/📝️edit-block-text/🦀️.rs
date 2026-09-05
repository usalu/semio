//! 📝 Note mutation — `EditBlockText`: replaces a text block's authored paragraph content.

use crate::artifacts::note::{NoteDiff, NoteSnapshot};
use crate::artifacts::note::schema::diff::note_block_patch_diff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 📝 `edit-block-text` payload — replaces a text block's authored paragraph content.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "edit-block-text")]
pub struct EditBlockText {
    pub id: String,
    pub new_paragraphs: Vec<crate::artifacts::note::NoteTextParagraph>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn edit_block_text(id: String, new_paragraphs: Vec<crate::artifacts::note::NoteTextParagraph>) -> NoteMutation {
    NoteMutation::EditBlockText(EditBlockText { id, new_paragraphs })
}

impl MutationKind<NoteSnapshot, NoteMutation> for EditBlockText {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "edit", entity: "block-text", kind: "edit-block-text", record: "EditedBlockText" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Edit block \"{}\" text", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

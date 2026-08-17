//! 📝 Note mutation — `EditBlockText`: replaces a text block's authored paragraph content.
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📝 `edit-block-text` payload — replaces a text block's authored paragraph content.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "edit-block-text")]
pub struct EditBlockText {
    pub id: String,
    pub new_paragraphs: Vec<crate::artifacts::note::NoteTextParagraph>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_block_text(id: String, new_paragraphs: Vec<crate::artifacts::note::NoteTextParagraph>) -> NoteMutation {
    NoteMutation::EditBlockText(EditBlockText { id, new_paragraphs })
}

impl MutationKind<NoteSnapshot, NoteMutation> for EditBlockText {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "edit", entity: "block-text", kind: "edit-block-text", record: "EditedBlockText" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Edit block \"{}\" text", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

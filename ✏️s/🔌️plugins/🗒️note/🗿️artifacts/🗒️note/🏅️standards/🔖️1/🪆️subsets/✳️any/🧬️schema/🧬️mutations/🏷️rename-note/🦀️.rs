//! 🏷️ Note mutation — `RenameNote`: sets the document's title.

use crate::artifacts::note::{NoteDiff, NoteSnapshot};
use crate::artifacts::note::schema::mutations::NoteMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🏷️ `rename-note` payload — sets the document's title.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "rename-note")]
pub struct RenameNote {
    pub new_title: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn rename_note(new_title: Option<String>) -> NoteMutation {
    NoteMutation::RenameNote(RenameNote { new_title })
}

impl MutationKind<NoteSnapshot, NoteMutation> for RenameNote {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "title", kind: "rename-note", record: "RenamedNote" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Rename note to {:?}", self.new_title)
    }
    async fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation

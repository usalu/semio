//! 🔤 Note mutation — `ChangeBlockFontSize`: sets a text block's font size.

use crate::artifacts::note::{NoteDiff, NoteSnapshot};
use crate::artifacts::note::schema::diff::note_block_patch_diff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔤 `change-block-font-size` payload — sets a text block's font size.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-block-font-size")]
pub struct ChangeBlockFontSize {
    pub id: String,
    pub new_font_size: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_block_font_size(id: String, new_font_size: f64) -> NoteMutation {
    NoteMutation::ChangeBlockFontSize(ChangeBlockFontSize { id, new_font_size })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ChangeBlockFontSize {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "block-font-size", kind: "change-block-font-size", record: "ChangedBlockFontSize" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change block \"{}\" font size to {}", self.id, self.new_font_size)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

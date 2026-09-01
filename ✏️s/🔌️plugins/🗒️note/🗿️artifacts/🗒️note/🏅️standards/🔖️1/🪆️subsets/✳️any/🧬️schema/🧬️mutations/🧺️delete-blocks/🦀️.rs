//! 🧺 Note mutation — `DeleteBlocks`: removes several blocks at once (multi-select delete).

use crate::artifacts::note::{NoteDiff, NoteSnapshot};
use crate::artifacts::note::schema::diff::note_block_removed_diff;
use crate::artifacts::note::schema::mutations::{CreateBlock, NoteMutation};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧺 `delete-blocks` payload — removes several blocks at once (multi-select delete).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-blocks")]
pub struct DeleteBlocks {
    pub ids: Vec<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn delete_blocks(ids: Vec<String>) -> NoteMutation {
    NoteMutation::DeleteBlocks(DeleteBlocks { ids })
}

impl MutationKind<NoteSnapshot, NoteMutation> for DeleteBlocks {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "blocks", kind: "delete-blocks", record: "DeletedBlocks" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete {} blocks", self.ids.len())
    }
    async fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
//#endregion 🔖️Mutation

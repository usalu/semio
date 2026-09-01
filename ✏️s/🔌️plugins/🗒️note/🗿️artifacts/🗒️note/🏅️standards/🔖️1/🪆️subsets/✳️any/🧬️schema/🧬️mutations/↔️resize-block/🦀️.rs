//! ↔️ Note mutation — `ResizeBlock`: changes a block's extent.

use crate::artifacts::note::{NoteDiff, NoteSnapshot};
use crate::artifacts::note::schema::diff::note_block_patch_diff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ↔️ `resize-block` payload — changes a block's extent.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "resize-block")]
pub struct ResizeBlock {
    pub id: String,
    pub new_width: f64,
    pub new_height: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn resize_block(id: String, new_width: f64, new_height: f64) -> NoteMutation {
    NoteMutation::ResizeBlock(ResizeBlock { id, new_width, new_height })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ResizeBlock {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "block", kind: "resize-block", record: "ResizedBlock" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Resize block \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

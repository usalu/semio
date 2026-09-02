//! 🔖 Note mutation — `RenameBlock`: sets a block's display name.

use crate::artifacts::note::{NoteDiff, NoteSnapshot};
use crate::artifacts::note::schema::diff::note_block_patch_diff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔖 `rename-block` payload — sets a block's display name.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "rename-block")]
pub struct RenameBlock {
    pub id: String,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn rename_block(id: String, new_name: String) -> NoteMutation {
    NoteMutation::RenameBlock(RenameBlock { id, new_name })
}

impl MutationKind<NoteSnapshot, NoteMutation> for RenameBlock {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "block", kind: "rename-block", record: "RenamedBlock" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Rename block to \"{}\"", self.new_name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

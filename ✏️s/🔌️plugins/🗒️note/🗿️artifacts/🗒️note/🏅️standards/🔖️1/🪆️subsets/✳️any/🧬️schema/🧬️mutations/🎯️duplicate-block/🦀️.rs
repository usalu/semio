//! 🎯 Note mutation — `DuplicateBlock`: copies a block to a new identity, placed after its source.

use crate::artifacts::note::{NoteDiff, NoteSnapshot};
use crate::artifacts::note::schema::diff::note_block_added_diff;
use crate::artifacts::note::schema::mutations::{DeleteBlock, NoteMutation};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🎯 `duplicate-block` payload — copies a block to a new identity, placed after its source.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "duplicate-block")]
pub struct DuplicateBlock {
    pub source_id: String,
    #[dsl(statements, block)]
    pub block: Box<crate::artifacts::note::NoteBlockNode>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn duplicate_block(source_id: String, block: crate::artifacts::note::NoteBlockNode) -> NoteMutation {
    NoteMutation::DuplicateBlock(DuplicateBlock { source_id, block: Box::new(block) })
}

impl MutationKind<NoteSnapshot, NoteMutation> for DuplicateBlock {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "duplicate", entity: "block", kind: "duplicate-block", record: "DuplicatedBlock" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Duplicate block \"{}\"", self.source_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.source_id.clone(), crate::artifacts::note::schema::block_id(&self.block).to_string()]
    }
}
//#endregion 🔖️Mutation

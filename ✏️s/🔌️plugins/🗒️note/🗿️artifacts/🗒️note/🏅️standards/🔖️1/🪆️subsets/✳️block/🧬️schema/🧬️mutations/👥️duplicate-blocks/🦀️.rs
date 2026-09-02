//! 👥 Note mutation — `DuplicateBlocks`: copies several blocks at once (multi-select duplicate).

use crate::artifacts::note::{NoteDiff, NoteSnapshot};
use crate::artifacts::note::schema::mutations::{DeleteBlocks, NoteMutation};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 👥 `duplicate-blocks` payload — copies several blocks at once (multi-select duplicate).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "duplicate-blocks")]
pub struct DuplicateBlocks {
    pub source_ids: Vec<String>,
    #[dsl(statements, block)]
    pub blocks: Vec<crate::artifacts::note::NoteBlockNode>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn duplicate_blocks(source_ids: Vec<String>, blocks: Vec<crate::artifacts::note::NoteBlockNode>) -> NoteMutation {
    NoteMutation::DuplicateBlocks(DuplicateBlocks { source_ids, blocks })
}

impl MutationKind<NoteSnapshot, NoteMutation> for DuplicateBlocks {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "duplicate", entity: "blocks", kind: "duplicate-blocks", record: "DuplicatedBlocks" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Duplicate {} blocks", self.source_ids.len())
    }
    async fn target(&self) -> Vec<String> {
        self.source_ids.clone()
    }
}
//#endregion 🔖️Mutation

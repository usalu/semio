//! 👥 Note mutation — `DuplicateBlocks`: copies several blocks at once (multi-select duplicate).
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 👥 `duplicate-blocks` payload — copies several blocks at once (multi-select duplicate).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "duplicate-blocks")]
pub struct DuplicateBlocks {
    pub source_ids: Vec<String>,
    #[dsl(statements, block)]
    pub blocks: Vec<crate::artifacts::note::NoteBlockNode>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn duplicate_blocks(source_ids: Vec<String>, blocks: Vec<crate::artifacts::note::NoteBlockNode>) -> NoteMutation {
    NoteMutation::DuplicateBlocks(DuplicateBlocks { source_ids, blocks })
}

impl MutationKind<NoteSnapshot, NoteMutation> for DuplicateBlocks {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "duplicate", entity: "blocks", kind: "duplicate-blocks", record: "DuplicatedBlocks" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Duplicate {} blocks", self.source_ids.len())
    }
    fn target(&self) -> Vec<String> {
        self.source_ids.clone()
    }
}
//#endregion 🔖️Mutation

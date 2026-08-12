//! 🎯 Note mutation — `DuplicateBlock`: copies a block to a new identity, placed after its source.
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🎯 `duplicate-block` payload — copies a block to a new identity, placed after its source.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "duplicate-block")]
pub struct DuplicateBlock {
    pub source_id: String,
    #[dsl(statements, block)]
    pub block: Box<crate::artifacts::note::NoteBlockNode>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn duplicate_block(source_id: String, block: crate::artifacts::note::NoteBlockNode) -> NoteMutation {
    NoteMutation::DuplicateBlock(DuplicateBlock { source_id, block: Box::new(block) })
}

impl MutationKind<NoteSnapshot, NoteMutation> for DuplicateBlock {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "duplicate", entity: "block", kind: "duplicate-block", record: "DuplicatedBlock" };

    fn diff(&self, base: &NoteSnapshot) -> NoteDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Duplicate block \"{}\"", self.source_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.source_id.clone(), crate::artifacts::note::engine::block_id(&self.block).to_string()]
    }
}
//#endregion 🔖️Mutation

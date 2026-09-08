//! 🎯 Note mutation — `DuplicateBlock`: copies a block to a new identity, placed after its source.

use crate::{NoteDiff, NoteSnapshot};
use crate::schema::mutations::NoteMutation;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🎯 `duplicate-block` payload — copies a block to a new identity, placed after its source.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "duplicate-block")]
pub struct DuplicateBlock {
    pub source_id: String,
    #[dsl(statements, block)]
    pub block: Box<crate::NoteBlockNode>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn duplicate_block(source_id: String, block: crate::NoteBlockNode) -> NoteMutation {
    NoteMutation::DuplicateBlock(DuplicateBlock { source_id, block: Box::new(block) })
}

impl MutationKind<NoteSnapshot, NoteMutation> for DuplicateBlock {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "duplicate", entity: "block", kind: "duplicate-block", record: "DuplicatedBlock" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Duplicate block \"{}\"", self.source_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.source_id.clone(), crate::schema::block_id(&self.block).to_string()]
    }
}
//#endregion 🔖️Mutation

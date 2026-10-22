//! 🎯 Note mutation — `DuplicateBlock`: copies a block to a new identity, placed after its source.

use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🎯 `duplicate-block` payload — copies a block to a new identity, placed after its source.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "duplicate-block")]
pub struct DuplicateBlock {
    pub source_id: String,
    #[dsl(statements, block)]
    pub block: Box<crate::NoteBlockNode>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn duplicate_block<Op>(source_id: String, block: crate::NoteBlockNode) -> Op
where
    Op: From<crate::standards::v1::subsets::block::schema::mutations::duplicate_block::DuplicateBlock>,
{
    Op::from(DuplicateBlock { source_id, block: Box::new(block) })
}

impl<Op> MutationKind<NoteSnapshot, Op> for DuplicateBlock
where
    Op: protocol::Mutation<NoteSnapshot, Diff = NoteDiff> + From<DuplicateBlock> + From<crate::standards::v1::subsets::block::schema::mutations::delete_block::DeleteBlock>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "duplicate", entity: "block", kind: "duplicate-block", record: "DuplicatedBlock" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Duplicate block \"{}\"", self.source_id), &format!("Block \"{}\" duplizieren", self.source_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.source_id.clone(), crate::schema::block_id(&self.block).to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<NoteSnapshot, Op> for DuplicateBlock
where
    Op: protocol::ColdMutation<NoteSnapshot, ReferenceDiff = NoteDiff> + From<DuplicateBlock> + From<crate::standards::v1::subsets::block::schema::mutations::delete_block::DeleteBlock>,
{
    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

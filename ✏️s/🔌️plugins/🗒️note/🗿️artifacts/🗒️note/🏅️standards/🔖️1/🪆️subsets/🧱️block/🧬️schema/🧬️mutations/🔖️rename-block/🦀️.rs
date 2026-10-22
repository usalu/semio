//! 🔖 Note mutation — `RenameBlock`: sets a block's display name.

use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔖 `rename-block` payload — sets a block's display name.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "rename-block")]
pub struct RenameBlock {
    pub id: String,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_block<Op>(id: String, new_name: String) -> Op
where
    Op: From<crate::standards::v1::subsets::block::schema::mutations::rename_block::RenameBlock>,
{
    Op::from(RenameBlock { id, new_name })
}

impl<Op> MutationKind<NoteSnapshot, Op> for RenameBlock
where
    Op: protocol::Mutation<NoteSnapshot, Diff = NoteDiff> + From<RenameBlock>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "block", kind: "rename-block", record: "RenamedBlock" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename block to \"{}\"", self.new_name), &format!("Block in \"{}\" umbenennen", self.new_name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<NoteSnapshot, Op> for RenameBlock
where
    Op: protocol::ColdMutation<NoteSnapshot, ReferenceDiff = NoteDiff> + From<RenameBlock>,
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

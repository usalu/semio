//! 🔒 Note mutation — `ChangeBlockLocked`: sets a block's locked state.

use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔒 `change-block-locked` payload — sets a block's locked state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-block-locked")]
pub struct ChangeBlockLocked {
    pub id: String,
    pub new_locked: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_block_locked<Op>(id: String, new_locked: bool) -> Op
where
    Op: From<crate::standards::v1::subsets::block::schema::mutations::change_block_locked::ChangeBlockLocked>,
{
    Op::from(ChangeBlockLocked { id, new_locked })
}

impl<Op> MutationKind<NoteSnapshot, Op> for ChangeBlockLocked
where
    Op: protocol::Mutation<NoteSnapshot, Diff = NoteDiff> + From<ChangeBlockLocked>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "block-locked", kind: "change-block-locked", record: "ChangedBlockLocked" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change block \"{}\" locked to {}", self.id, self.new_locked), &format!("Sperre von Block \"{}\" auf {} ändern", self.id, self.new_locked))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<NoteSnapshot, Op> for ChangeBlockLocked
where
    Op: protocol::ColdMutation<NoteSnapshot, ReferenceDiff = NoteDiff> + From<ChangeBlockLocked>,
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

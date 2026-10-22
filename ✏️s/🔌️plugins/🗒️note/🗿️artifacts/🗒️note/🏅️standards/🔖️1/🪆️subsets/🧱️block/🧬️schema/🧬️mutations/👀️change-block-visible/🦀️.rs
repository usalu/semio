//! 👀 Note mutation — `ChangeBlockVisible`: sets a block's visibility.

use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 👀 `change-block-visible` payload — sets a block's visibility.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-block-visible")]
pub struct ChangeBlockVisible {
    pub id: String,
    pub new_visible: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_block_visible<Op>(id: String, new_visible: bool) -> Op
where
    Op: From<crate::standards::v1::subsets::block::schema::mutations::change_block_visible::ChangeBlockVisible>,
{
    Op::from(ChangeBlockVisible { id, new_visible })
}

impl<Op> MutationKind<NoteSnapshot, Op> for ChangeBlockVisible
where
    Op: protocol::Mutation<NoteSnapshot, Diff = NoteDiff> + From<ChangeBlockVisible>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "block-visible", kind: "change-block-visible", record: "ChangedBlockVisible" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change block \"{}\" visible to {}", self.id, self.new_visible), &format!("Sichtbarkeit von Block \"{}\" auf {} ändern", self.id, self.new_visible))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<NoteSnapshot, Op> for ChangeBlockVisible
where
    Op: protocol::ColdMutation<NoteSnapshot, ReferenceDiff = NoteDiff> + From<ChangeBlockVisible>,
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

//! 🧮 Note mutation — `EditBlockMath`: replaces a math block's authored TeX source.

use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧮 `edit-block-math` payload — replaces a math block's authored TeX source.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "edit-block-math")]
pub struct EditBlockMath {
    pub id: String,
    pub new_tex: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_block_math<Op>(id: String, new_tex: String) -> Op
where
    Op: From<crate::standards::v1::subsets::math::schema::mutations::edit_block_math::EditBlockMath>,
{
    Op::from(EditBlockMath { id, new_tex })
}

impl<Op> MutationKind<NoteSnapshot, Op> for EditBlockMath
where
    Op: protocol::Mutation<NoteSnapshot, Diff = NoteDiff> + From<EditBlockMath>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "edit", entity: "block-math", kind: "edit-block-math", record: "EditedBlockMath" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit block \"{}\" math", self.id), &format!("Formel von Block \"{}\" bearbeiten", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<NoteSnapshot, Op> for EditBlockMath
where
    Op: protocol::ColdMutation<NoteSnapshot, ReferenceDiff = NoteDiff> + From<EditBlockMath>,
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

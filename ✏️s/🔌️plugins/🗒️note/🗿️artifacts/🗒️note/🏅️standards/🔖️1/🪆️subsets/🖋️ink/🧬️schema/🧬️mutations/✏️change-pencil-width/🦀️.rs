//! ✏️ Note mutation — `ChangePencilWidth`: sets the pencil stroke width.

use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✏️ `change-pencil-width` payload — sets the pencil stroke width.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-pencil-width")]
pub struct ChangePencilWidth {
    pub new_width: Option<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_pencil_width<Op>(new_width: Option<f64>) -> Op
where
    Op: From<crate::standards::v1::subsets::ink::schema::mutations::change_pencil_width::ChangePencilWidth>,
{
    Op::from(ChangePencilWidth { new_width })
}

impl<Op> MutationKind<NoteSnapshot, Op> for ChangePencilWidth
where
    Op: protocol::Mutation<NoteSnapshot, Diff = NoteDiff> + From<ChangePencilWidth>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "pencil-width", kind: "change-pencil-width", record: "ChangedPencilWidth" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        crate::schema::note_setting_label(("pencil width", "Stiftbreite"), self.new_width.map(crate::schema::note_label_number))
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<NoteSnapshot, Op> for ChangePencilWidth
where
    Op: protocol::ColdMutation<NoteSnapshot, ReferenceDiff = NoteDiff> + From<ChangePencilWidth>,
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

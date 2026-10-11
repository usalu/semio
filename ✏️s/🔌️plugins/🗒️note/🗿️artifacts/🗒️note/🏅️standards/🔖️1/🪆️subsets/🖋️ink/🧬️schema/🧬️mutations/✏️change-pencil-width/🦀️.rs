//! ✏️ Note mutation — `ChangePencilWidth`: sets the pencil stroke width.

use crate::schema::mutations::NoteMutation;
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
pub fn change_pencil_width(new_width: Option<f64>) -> NoteMutation {
    NoteMutation::ChangePencilWidth(ChangePencilWidth { new_width })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ChangePencilWidth {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "pencil-width", kind: "change-pencil-width", record: "ChangedPencilWidth" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        crate::schema::mutations::note_setting_label(("pencil width", "Stiftbreite"), self.new_width.map(crate::schema::mutations::note_label_number))
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation

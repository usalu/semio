//! 🏷️ Note mutation — `RenameNote`: sets the document's title.

use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🏷️ `rename-note` payload — sets the document's title.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "rename-note")]
pub struct RenameNote {
    pub new_title: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_note<Op>(new_title: Option<String>) -> Op
where
    Op: From<crate::standards::v1::subsets::document::schema::mutations::rename_note::RenameNote>,
{
    Op::from(RenameNote { new_title })
}

impl<Op> MutationKind<NoteSnapshot, Op> for RenameNote
where
    Op: protocol::Mutation<NoteSnapshot, Diff = NoteDiff> + From<RenameNote>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "title", kind: "rename-note", record: "RenamedNote" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        match &self.new_title {
            Some(title) => semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename note to \"{title}\""), &format!("Notiz in \"{title}\" umbenennen")),
            None => semio_framework_ui_locale::LocalizedLabel::native("Remove note title", "Notiztitel entfernen"),
        }
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation

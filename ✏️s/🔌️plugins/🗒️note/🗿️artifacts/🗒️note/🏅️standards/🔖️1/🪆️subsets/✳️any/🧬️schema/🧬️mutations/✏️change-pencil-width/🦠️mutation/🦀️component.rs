//! ✏️ Note mutation — `ChangePencilWidth`: sets the pencil stroke width.
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✏️ `change-pencil-width` payload — sets the pencil stroke width.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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

    fn diff(&self, base: &NoteSnapshot) -> NoteDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change pencil width to {:?}", self.new_width)
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation

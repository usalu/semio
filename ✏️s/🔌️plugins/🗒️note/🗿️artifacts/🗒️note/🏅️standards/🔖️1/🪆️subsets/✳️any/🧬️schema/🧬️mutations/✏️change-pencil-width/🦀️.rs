//! ✏️ Note mutation — `ChangePencilWidth`: sets the pencil stroke width.

use crate::artifacts::note::{NoteDiff, NoteSnapshot};
use crate::artifacts::note::schema::mutations::NoteMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✏️ `change-pencil-width` payload — sets the pencil stroke width.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-pencil-width")]
pub struct ChangePencilWidth {
    pub new_width: Option<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_pencil_width(new_width: Option<f64>) -> NoteMutation {
    NoteMutation::ChangePencilWidth(ChangePencilWidth { new_width })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ChangePencilWidth {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "pencil-width", kind: "change-pencil-width", record: "ChangedPencilWidth" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change pencil width to {:?}", self.new_width)
    }
    async fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation

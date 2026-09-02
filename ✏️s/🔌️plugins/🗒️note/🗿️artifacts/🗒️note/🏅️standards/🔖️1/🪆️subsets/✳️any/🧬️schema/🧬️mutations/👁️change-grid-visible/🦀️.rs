//! 👁️ Note mutation — `ChangeGridVisible`: sets grid visibility.

use crate::artifacts::note::{NoteDiff, NoteSnapshot};
use crate::artifacts::note::schema::mutations::NoteMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 👁️ `change-grid-visible` payload — sets grid visibility.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-grid-visible")]
pub struct ChangeGridVisible {
    pub new_visible: Option<bool>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_grid_visible(new_visible: Option<bool>) -> NoteMutation {
    NoteMutation::ChangeGridVisible(ChangeGridVisible { new_visible })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ChangeGridVisible {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "grid-visible", kind: "change-grid-visible", record: "ChangedGridVisible" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change grid visible to {:?}", self.new_visible)
    }
    async fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation

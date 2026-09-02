//! 🌫️ Note mutation — `ChangeGridOpacity`: sets grid opacity.

use crate::artifacts::note::{NoteDiff, NoteSnapshot};
use crate::artifacts::note::schema::mutations::NoteMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🌫️ `change-grid-opacity` payload — sets grid opacity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-grid-opacity")]
pub struct ChangeGridOpacity {
    pub new_opacity: Option<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_grid_opacity(new_opacity: Option<f64>) -> NoteMutation {
    NoteMutation::ChangeGridOpacity(ChangeGridOpacity { new_opacity })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ChangeGridOpacity {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "grid-opacity", kind: "change-grid-opacity", record: "ChangedGridOpacity" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change grid opacity to {:?}", self.new_opacity)
    }
    async fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation

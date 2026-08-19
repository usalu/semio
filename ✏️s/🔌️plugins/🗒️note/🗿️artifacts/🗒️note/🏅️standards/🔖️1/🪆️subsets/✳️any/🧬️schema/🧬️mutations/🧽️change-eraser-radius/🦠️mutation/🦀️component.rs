//! 🧽 Note mutation — `ChangeEraserRadius`: sets the eraser radius.
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧽 `change-eraser-radius` payload — sets the eraser radius.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-eraser-radius")]
pub struct ChangeEraserRadius {
    pub new_radius: Option<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_eraser_radius(new_radius: Option<f64>) -> NoteMutation {
    NoteMutation::ChangeEraserRadius(ChangeEraserRadius { new_radius })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ChangeEraserRadius {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "eraser-radius", kind: "change-eraser-radius", record: "ChangedEraserRadius" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change eraser radius to {:?}", self.new_radius)
    }
    async fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation

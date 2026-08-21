//! 📐 Note mutation — `ChangeSnapGridSpacing`: sets the snap grid spacing.
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📐 `change-snap-grid-spacing` payload — sets the snap grid spacing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-snap-grid-spacing")]
pub struct ChangeSnapGridSpacing {
    pub new_spacing: Option<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_snap_grid_spacing(new_spacing: Option<f64>) -> NoteMutation {
    NoteMutation::ChangeSnapGridSpacing(ChangeSnapGridSpacing { new_spacing })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ChangeSnapGridSpacing {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "snap-grid-spacing", kind: "change-snap-grid-spacing", record: "ChangedSnapGridSpacing" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change snap grid spacing to {:?}", self.new_spacing)
    }
    async fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation

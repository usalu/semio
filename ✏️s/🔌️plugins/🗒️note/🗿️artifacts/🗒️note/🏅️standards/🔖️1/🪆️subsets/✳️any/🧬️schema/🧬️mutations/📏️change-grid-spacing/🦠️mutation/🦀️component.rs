//! 📏 Note mutation — `ChangeGridSpacing`: sets grid spacing.
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📏 `change-grid-spacing` payload — sets grid spacing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-grid-spacing")]
pub struct ChangeGridSpacing {
    pub new_spacing: Option<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_grid_spacing(new_spacing: Option<f64>) -> NoteMutation {
    NoteMutation::ChangeGridSpacing(ChangeGridSpacing { new_spacing })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ChangeGridSpacing {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "grid-spacing", kind: "change-grid-spacing", record: "ChangedGridSpacing" };

    fn diff(&self, base: &NoteSnapshot) -> NoteDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change grid spacing to {:?}", self.new_spacing)
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation

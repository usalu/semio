//! 🌫️ Note mutation — `ChangeGridOpacity`: sets grid opacity.
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌫️ `change-grid-opacity` payload — sets grid opacity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-grid-opacity")]
pub struct ChangeGridOpacity {
    pub new_opacity: Option<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_grid_opacity(new_opacity: Option<f64>) -> NoteMutation {
    NoteMutation::ChangeGridOpacity(ChangeGridOpacity { new_opacity })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ChangeGridOpacity {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "grid-opacity", kind: "change-grid-opacity", record: "ChangedGridOpacity" };

    fn diff(&self, base: &NoteSnapshot) -> NoteDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change grid opacity to {:?}", self.new_opacity)
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation

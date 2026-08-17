//! 🔢 Note mutation — `ChangeGridSubdivisions`: sets grid subdivisions.
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔢 `change-grid-subdivisions` payload — sets grid subdivisions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-grid-subdivisions")]
pub struct ChangeGridSubdivisions {
    pub new_subdivisions: Option<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_grid_subdivisions(new_subdivisions: Option<f64>) -> NoteMutation {
    NoteMutation::ChangeGridSubdivisions(ChangeGridSubdivisions { new_subdivisions })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ChangeGridSubdivisions {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "grid-subdivisions", kind: "change-grid-subdivisions", record: "ChangedGridSubdivisions" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change grid subdivisions to {:?}", self.new_subdivisions)
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation

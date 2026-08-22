//! Puzzle3d mutation — `EditObjectLabel`: replaces an object's authored display label.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `edit-object-label` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "edit-object-label")]
pub struct EditObjectLabel {
    pub id: String,
    pub new_label: Option<String>,
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for EditObjectLabel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "object", kind: "edit-object-label", record: "EditedObjectLabel" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Edit object \"{}\" label", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_object_label(id: String, new_label: Option<String>) -> Puzzle3dMutation {
    Puzzle3dMutation::EditObjectLabel(EditObjectLabel { id, new_label })
}

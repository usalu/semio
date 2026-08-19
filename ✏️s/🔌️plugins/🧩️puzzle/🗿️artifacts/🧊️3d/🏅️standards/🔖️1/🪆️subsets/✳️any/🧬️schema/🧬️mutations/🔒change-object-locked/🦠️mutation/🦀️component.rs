//! Puzzle3d mutation — `ChangeObjectLocked`: changes an object's locked flag.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `change-object-locked` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-object-locked")]
pub struct ChangeObjectLocked {
    pub id: String,
    pub new_locked: bool,
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for ChangeObjectLocked {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "object", kind: "change-object-locked", record: "ChangedObjectLocked" };

    async fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change object \"{}\" locked", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_object_locked(id: String, new_locked: bool) -> Puzzle3dMutation {
    Puzzle3dMutation::ChangeObjectLocked(ChangeObjectLocked { id, new_locked })
}

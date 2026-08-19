//! Puzzle3d mutation — `ChangeObjectKind`: changes an object's `object_kind` catalog reference.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `change-object-kind` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-object-kind")]
pub struct ChangeObjectKind {
    pub id: String,
    pub new_object_kind: Option<String>,
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for ChangeObjectKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "object", kind: "change-object-kind", record: "ChangedObjectKind" };

    async fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change object \"{}\" kind", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_object_kind(id: String, new_object_kind: Option<String>) -> Puzzle3dMutation {
    Puzzle3dMutation::ChangeObjectKind(ChangeObjectKind { id, new_object_kind })
}

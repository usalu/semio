//! Puzzle3d mutation — `ResizeReference`: changes a reference plane's world-space width.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `resize-reference` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "resize-reference")]
pub struct ResizeReference {
    pub id: String,
    pub new_width_world: f64,
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for ResizeReference {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "resize", entity: "reference", kind: "resize-reference", record: "ResizedReference" };

    async fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Resize reference \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn resize_reference(id: String, new_width_world: f64) -> Puzzle3dMutation {
    Puzzle3dMutation::ResizeReference(ResizeReference { id, new_width_world })
}

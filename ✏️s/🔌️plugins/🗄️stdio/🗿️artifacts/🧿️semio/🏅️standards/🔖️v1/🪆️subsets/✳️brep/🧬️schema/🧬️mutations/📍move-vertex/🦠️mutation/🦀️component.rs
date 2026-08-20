//! 📍 `move-vertex` — absolute spatial reposition of a vertex (taxonomy's `move` verb — FINAL-state
//! absolute position, not a relative offset). SMO approved this verb explicitly.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveVertex {
    pub vertex_id: String,
    pub new_point: SemioPoint3,
}

impl protocol::MutationKind<SemioBrepSnapshot, SemioBrepMutation> for MoveVertex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "vertex", kind: "move-vertex", record: "MovedVertex" };

    async fn diff(&self, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<<SemioBrepMutation as protocol::Mutation<SemioBrepSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Move vertex \"{}\" to ({}, {}, {})", self.vertex_id, self.new_point.x, self.new_point.y, self.new_point.z)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.vertex_id.clone()]
    }
}
//#endregion 🔖️Payload

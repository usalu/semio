//! 📍 `move-vertex` — absolute spatial reposition of ONE vertex inside a primitive's `positions` buffer, addressed by BASE-state index (`positions` only ever changes wholesale via `replace-primitive-geometry`, so an index stays valid for the lifetime of one geometry \"epoch\"). Present in the vocabulary because `computed-normals` (deliberately OMITTED from this subset's own inference facet, see its module doc comment) would have presumed raw per-vertex position edits are a first-class gesture; `move-vertex` is authored on its own independent merits — a real address (`mesh_id`+`primitive_id`+`vertex_index`) and one field (`new_point`) — matching taxonomy's `move` verb exactly, same shape as brep's own approved `move-vertex`.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveVertex {
    pub mesh_id: String,
    pub primitive_id: String,
    pub vertex_index: usize,
    pub new_point: SemioPoint3,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for MoveVertex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "vertex", kind: "move-vertex", record: "MovedVertex" };

    async fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Move vertex {} of primitive \"{}\" in mesh \"{}\" to ({}, {}, {})", self.vertex_index, self.primitive_id, self.mesh_id, self.new_point.x, self.new_point.y, self.new_point.z)
    }
    async fn target(&self) -> Vec<String> {
        vec![format!("{}:{}:{}", self.mesh_id, self.primitive_id, self.vertex_index)]
    }
}
//#endregion 🔖️Payload

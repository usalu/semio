//! 🕸️ `create-mesh` — brings a new id-keyed mesh into existence. A duplicate `id` already present in `base` is a no-op (never a duplicate id).

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMesh;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateMesh {
    pub mesh: SemioMesh,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for CreateMesh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "mesh", kind: "create-mesh", record: "CreatedMesh" };

    async fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create mesh \"{}\"", self.mesh.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.mesh.id.clone()]
    }
}
//#endregion 🔖️Payload

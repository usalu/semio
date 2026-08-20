//! 🖼️ `create-texture` — brings a new id-keyed texture into existence. A duplicate `id` already present in `base` is a no-op.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioTexture;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTexture {
    pub texture: SemioTexture,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for CreateTexture {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "texture", kind: "create-texture", record: "CreatedTexture" };

    async fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Create texture \"{}\"", self.texture.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.texture.id.clone()]
    }
}
//#endregion 🔖️Payload

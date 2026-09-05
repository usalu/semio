//! 🖼️ `create-texture` — brings a new id-keyed texture into existence. A duplicate `id` already present in `base` is a no-op.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::{SemioMeshMutation, delete_texture};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMeshSnapshot, SemioTexture};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateTexture {
    pub texture: SemioTexture,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for CreateTexture {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "texture", kind: "create-texture", record: "CreatedTexture" };

    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create texture \"{}\"", self.texture.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.texture.id.clone()]
    }
}
//#endregion 🔖️Payload

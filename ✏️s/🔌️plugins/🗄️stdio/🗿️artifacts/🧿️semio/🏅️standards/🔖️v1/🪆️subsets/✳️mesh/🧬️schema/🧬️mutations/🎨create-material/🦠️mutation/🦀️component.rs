//! 🎨 `create-material` — brings a new id-keyed material into existence. A duplicate `id` already present in `base` is a no-op.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMaterial;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateMaterial {
    pub material: SemioMaterial,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for CreateMaterial {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "material", kind: "create-material", record: "CreatedMaterial" };

    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create material \"{}\"", self.material.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.material.id.clone()]
    }
}
//#endregion 🔖️Payload

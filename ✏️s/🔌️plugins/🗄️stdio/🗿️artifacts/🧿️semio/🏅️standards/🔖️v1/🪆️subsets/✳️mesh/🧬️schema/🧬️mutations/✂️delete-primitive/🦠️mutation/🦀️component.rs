//! ✂️ `delete-primitive` — removes primitive `primitive_id` from mesh `mesh_id`.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeletePrimitive {
    pub mesh_id: String,
    pub primitive_id: String,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for DeletePrimitive {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "primitive", kind: "delete-primitive", record: "DeletedPrimitive" };

    fn diff(&self, base: &SemioMeshSnapshot) -> <SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete primitive \"{}\" from mesh \"{}\"", self.primitive_id, self.mesh_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.primitive_id.clone()]
    }
}
//#endregion 🔖️Payload

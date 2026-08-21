//! 🔺 `create-primitive` — inserts a new id-keyed primitive into mesh `mesh_id`. A duplicate `primitive_id` already present, or an absent `mesh_id`, is a no-op.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioPrimitive;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePrimitive {
    pub mesh_id: String,
    pub primitive: SemioPrimitive,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for CreatePrimitive {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "primitive", kind: "create-primitive", record: "CreatedPrimitive" };

    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create primitive \"{}\" in mesh \"{}\"", self.primitive.id, self.mesh_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.primitive.id.clone()]
    }
}
//#endregion 🔖️Payload

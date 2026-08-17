//! 🚮 `delete-material` — removes an id-keyed material. Does NOT cascade to clear `material_id` references on primitives that pointed at it — `material_id` is a soft `Option<String>` reference with no membership-cascade verb (same category of honest gap brep's own `loop.edges` exclusion documents), and this matches the PRE-EXISTING behaviour of the old `RemoveMaterial` variant exactly, not a new gap introduced by this wave.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteMaterial {
    pub id: String,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for DeleteMaterial {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "material", kind: "delete-material", record: "DeletedMaterial" };

    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete material \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload

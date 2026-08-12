//! 🧱 `change-material-roughness` — sets a material's PBR roughness factor. See `change-material-metallic`'s sibling doc comment; the same decompose reasoning applies symmetrically.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeMaterialRoughness {
    pub id: String,
    pub new_roughness: f32,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for ChangeMaterialRoughness {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material-roughness", kind: "change-material-roughness", record: "ChangedMaterialRoughness" };

    fn diff(&self, base: &SemioMeshSnapshot) -> <SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change material \"{}\" roughness factor", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload

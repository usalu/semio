//! ⚙️ `change-material-metallic` — sets a material's PBR metallic factor. Decomposed from the old bundled `SetMaterialPbr{metallic,roughness}`: `metallic`/`roughness` are two independent top-level scalar fields (unlike `base_color`, grouped into one `SemioRgba` value type), and every real PBR editor sets them via two independent sliders — same decompose test SMO's `StrokeStyle` ruling already applies.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeMaterialMetallic {
    pub id: String,
    pub new_metallic: f32,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for ChangeMaterialMetallic {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material-metallic", kind: "change-material-metallic", record: "ChangedMaterialMetallic" };

    async fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change material \"{}\" metallic factor", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload

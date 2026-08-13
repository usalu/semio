//! 📐 `replace-primitive-geometry` — whole-value swap of a primitive's positions/normals/uvs/colors/indices buffer set. SMO-approved rename of the old `set-primitive-geometry`: a multi-array vertex-buffer blob is a structured sub-payload, so `set` was the wrong verb. SMO approved the reasoning and reserved the edit; SMO wound down without doing it; DKM completes it here.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioRgba, SemioUv};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplacePrimitiveGeometry {
    pub mesh_id: String,
    pub primitive_id: String,
    pub positions: Vec<SemioPoint3>,
    pub normals: Vec<SemioPoint3>,
    pub uvs: Vec<SemioUv>,
    pub colors: Vec<SemioRgba>,
    pub indices: Vec<u32>,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for ReplacePrimitiveGeometry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "primitive-geometry", kind: "replace-primitive-geometry", record: "ReplacedPrimitiveGeometry" };

    fn diff(&self, base: &SemioMeshSnapshot) -> <SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace primitive \"{}\" geometry in mesh \"{}\"", self.primitive_id, self.mesh_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.primitive_id.clone()]
    }
}
//#endregion 🔖️Payload

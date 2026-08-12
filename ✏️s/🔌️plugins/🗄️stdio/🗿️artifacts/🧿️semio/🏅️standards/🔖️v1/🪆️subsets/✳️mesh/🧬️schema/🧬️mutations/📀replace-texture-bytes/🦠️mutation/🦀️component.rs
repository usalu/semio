//! 📀 `replace-texture-bytes` — whole-value swap of a texture's raw byte payload. Raw image bytes are the \"large\" swapped payload (matches `replace-primitive-geometry`'s exact rename rationale), never edited byte-by-byte from outside, so `replace`, not `change`/`set`.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceTextureBytes {
    pub id: String,
    pub new_bytes: Vec<u8>,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for ReplaceTextureBytes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "texture-bytes", kind: "replace-texture-bytes", record: "ReplacedTextureBytes" };

    fn diff(&self, base: &SemioMeshSnapshot) -> <SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace texture \"{}\" bytes", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload

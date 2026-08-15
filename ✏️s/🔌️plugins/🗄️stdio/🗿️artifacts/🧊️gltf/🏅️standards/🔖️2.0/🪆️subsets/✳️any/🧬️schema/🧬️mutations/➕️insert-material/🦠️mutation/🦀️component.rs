//! 🦠️ `insert-material` GLTF mutation payload.

use super::super::planning::{reject, remap_references, GltfMutationRejection, GltfSemanticMutation, IndexFamily};
use crate::artifacts::gltf::schema::mutations::GltfMutation;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertMaterial {
    pub index: usize,
    pub material: GltfMaterial,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for InsertMaterial {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "material", kind: "insert-material", record: "InsertMaterial" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "InsertMaterial".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for InsertMaterial {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        let document = &mut snapshot.document;
        if self.index > document.materials.len() {
            return Err(reject("gltf.mutation.insert-out-of-range", "document/materials", format!("index {}, length {}", self.index, document.materials.len())));
        }
        remap_references(document, IndexFamily::Material, self.index, true);
        document.materials.insert(self.index, self.material.clone());
        Ok(())
    }
}

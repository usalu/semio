//! 🦠️ `insert-scene` GLTF mutation payload.

use super::super::planning::{reject, remap_references, GltfMutationRejection, GltfSemanticMutation, IndexFamily};
use crate::artifacts::gltf::schema::mutations::GltfMutation;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertScene {
    pub index: usize,
    pub scene: GltfScene,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for InsertScene {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "scene", kind: "insert-scene", record: "InsertScene" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "InsertScene".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for InsertScene {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        let document = &mut snapshot.document;
        if self.index > document.scenes.len() {
            return Err(reject("gltf.mutation.insert-out-of-range", "document/scenes", format!("index {}, length {}", self.index, document.scenes.len())));
        }
        remap_references(document, IndexFamily::Scene, self.index, true);
        document.scenes.insert(self.index, self.scene.clone());
        Ok(())
    }
}

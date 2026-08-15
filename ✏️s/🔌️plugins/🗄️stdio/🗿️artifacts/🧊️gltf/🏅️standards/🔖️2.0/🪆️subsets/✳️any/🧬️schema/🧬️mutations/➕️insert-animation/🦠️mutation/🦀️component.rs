//! 🦠️ `insert-animation` GLTF mutation payload.

use super::super::planning::{reject, GltfMutationRejection, GltfSemanticMutation};
use crate::artifacts::gltf::schema::mutations::GltfMutation;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertAnimation {
    pub index: usize,
    pub animation: GltfAnimation,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for InsertAnimation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "animation", kind: "insert-animation", record: "InsertAnimation" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "InsertAnimation".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for InsertAnimation {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        if self.index > snapshot.document.animations.len() {
            return Err(reject("gltf.mutation.insert-out-of-range", "document/animations", format!("index {}, length {}", self.index, snapshot.document.animations.len())));
        }
        snapshot.document.animations.insert(self.index, self.animation.clone());
        Ok(())
    }
}

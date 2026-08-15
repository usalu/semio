//! 🦠️ `bind-primitive-material` GLTF mutation payload.

use crate::artifacts::gltf::schema::mutations::GltfMutation;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BindPrimitiveMaterial {
    pub mesh: usize,
    pub primitive: usize,
    pub material: Option<usize>,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for BindPrimitiveMaterial {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "primitive-material", kind: "bind-primitive-material", record: "BindPrimitiveMaterial" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "BindPrimitiveMaterial".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.mesh.to_string(), self.primitive.to_string()]
    }
}

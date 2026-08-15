//! 🦠️ `transform-node` GLTF mutation payload.

use super::super::planning::{check_index, reject, GltfMutationRejection, GltfSemanticMutation};
use crate::artifacts::gltf::schema::mutations::GltfMutation;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformNode {
    pub index: usize,
    pub matrix: Option<[f64; 16]>,
    pub translation: Option<[f64; 3]>,
    pub rotation: Option<[f64; 4]>,
    pub scale: Option<[f64; 3]>,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for TransformNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "transform", entity: "node", kind: "transform-node", record: "TransformNode" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "TransformNode".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for TransformNode {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        check_index("document/nodes", self.index, snapshot.document.nodes.len())?;
        if self.matrix.is_some() && (self.translation.is_some() || self.rotation.is_some() || self.scale.is_some()) {
            return Err(reject("gltf.node.transform-exclusive", format!("document/nodes/{}", self.index), "matrix and TRS cannot coexist"));
        }
        if self.matrix.iter().flatten().chain(self.translation.iter().flatten()).chain(self.rotation.iter().flatten()).chain(self.scale.iter().flatten()).any(|value| !value.is_finite()) {
            return Err(reject("gltf.node.transform-nonfinite", format!("document/nodes/{}", self.index), "transform contains a non-finite number"));
        }
        let node = &mut snapshot.document.nodes[self.index];
        node.matrix = self.matrix;
        node.translation = self.translation;
        node.rotation = self.rotation;
        node.scale = self.scale;
        Ok(())
    }
}

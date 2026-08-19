//#region 🦠️Mutation
// 🦠️ `transform-node` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{check_index, reject, GltfMutationRejection, GltfSemanticMutation};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
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
    async fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    async fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "TransformNode".into()
    }
    async fn target(&self) -> Vec<String> {
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
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `transform-node` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::TransformNode;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub async fn diff(payload: &TransformNode, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `TransformNode` semantic inverse.
    
    use super::TransformNode;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub async fn inverse(payload: &TransformNode, base: &GltfSnapshot) -> Vec<GltfMutation> {
        base.document.nodes.get(payload.index).map(|node| vec![GltfMutation::TransformNode(TransformNode { index: payload.index, matrix: node.matrix, translation: node.translation, rotation: node.rotation, scale: node.scale })]).unwrap_or_default()
    }
}
//#endregion ↩️Inverse


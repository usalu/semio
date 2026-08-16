//#region 🦠️Mutation
// 🦠️ `insert-node` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{reject, remap_references, shift_insert, GltfMutationRejection, GltfSemanticMutation, IndexFamily};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertNode {
    pub index: usize,
    pub node: GltfNode,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for InsertNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "node", kind: "insert-node", record: "InsertNode" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "InsertNode".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for InsertNode {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        let document = &mut snapshot.document;
        if self.index > document.nodes.len() {
            return Err(reject("gltf.mutation.insert-out-of-range", "document/nodes", format!("index {}, length {}", self.index, document.nodes.len())));
        }
        remap_references(document, IndexFamily::Node, self.index, true);
        let mut node = self.node.clone();
        node.children.iter_mut().for_each(|child| shift_insert(child, self.index));
        document.nodes.insert(self.index, node);
        Ok(())
    }
}
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `insert-node` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::InsertNode;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn diff(payload: &InsertNode, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `InsertNode` semantic inverse.
    
    use super::InsertNode;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn inverse(payload: &InsertNode, _base: &GltfSnapshot) -> Vec<GltfMutation> {
        vec![GltfMutation::RemoveNode(RemoveNode { index: payload.index })]
    }
}
//#endregion ↩️Inverse


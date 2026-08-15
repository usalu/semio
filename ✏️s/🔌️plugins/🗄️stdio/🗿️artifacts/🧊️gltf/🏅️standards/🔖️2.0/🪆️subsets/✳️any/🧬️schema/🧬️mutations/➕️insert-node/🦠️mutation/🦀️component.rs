//! 🦠️ `insert-node` GLTF mutation payload.

use super::super::planning::{reject, remap_references, shift_insert, GltfMutationRejection, GltfSemanticMutation, IndexFamily};
use crate::artifacts::gltf::schema::mutations::GltfMutation;
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
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        super::inverse::inverse(self, base)
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

//! 🦠️ `insert-buffer` GLTF mutation payload.

use super::super::planning::{reject, remap_references, GltfMutationRejection, GltfSemanticMutation, IndexFamily};
use crate::artifacts::gltf::schema::mutations::GltfMutation;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertBuffer {
    pub index: usize,
    pub buffer: GltfBuffer,
    pub bytes: Vec<u8>,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for InsertBuffer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "buffer", kind: "insert-buffer", record: "InsertBuffer" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "InsertBuffer".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for InsertBuffer {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        if self.index > snapshot.document.buffers.len() {
            return Err(reject("gltf.mutation.insert-out-of-range", "document/buffers", format!("index {}, length {}", self.index, snapshot.document.buffers.len())));
        }
        remap_references(&mut snapshot.document, IndexFamily::Buffer, self.index, true);
        snapshot.document.buffers.insert(self.index, self.buffer.clone());
        snapshot.buffers.insert(self.index, self.bytes.clone());
        Ok(())
    }
}

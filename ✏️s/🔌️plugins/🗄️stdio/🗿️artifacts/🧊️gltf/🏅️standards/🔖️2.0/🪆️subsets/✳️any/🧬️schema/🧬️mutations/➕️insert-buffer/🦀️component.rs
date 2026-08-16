//#region 🦠️Mutation
// 🦠️ `insert-buffer` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{reject, remap_references, GltfMutationRejection, GltfSemanticMutation, IndexFamily};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
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
        diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
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
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `insert-buffer` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::InsertBuffer;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn diff(payload: &InsertBuffer, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `InsertBuffer` semantic inverse.
    
    use super::InsertBuffer;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn inverse(payload: &InsertBuffer, _base: &GltfSnapshot) -> Vec<GltfMutation> {
        vec![GltfMutation::RemoveBuffer(RemoveBuffer { index: payload.index })]
    }
}
//#endregion ↩️Inverse


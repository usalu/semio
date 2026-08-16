//#region 🦠️Mutation
// 🦠️ `set-buffer` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{check_index, GltfMutationRejection, GltfSemanticMutation};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetBuffer {
    pub index: usize,
    pub buffer: GltfBuffer,
    pub bytes: Vec<u8>,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for SetBuffer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "buffer", kind: "set-buffer", record: "SetBuffer" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "SetBuffer".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for SetBuffer {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        check_index("document/buffers", self.index, snapshot.document.buffers.len())?;
        snapshot.document.buffers[self.index] = self.buffer.clone();
        snapshot.buffers[self.index] = self.bytes.clone();
        Ok(())
    }
}
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `set-buffer` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::SetBuffer;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn diff(payload: &SetBuffer, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `SetBuffer` semantic inverse.
    
    use super::SetBuffer;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn inverse(payload: &SetBuffer, base: &GltfSnapshot) -> Vec<GltfMutation> {
        match (base.document.buffers.get(payload.index), base.buffers.get(payload.index)) {
            (Some(buffer), Some(bytes)) => vec![GltfMutation::SetBuffer(SetBuffer { index: payload.index, buffer: buffer.clone(), bytes: bytes.clone() })],
            _ => Vec::new(),
        }
    }
}
//#endregion ↩️Inverse


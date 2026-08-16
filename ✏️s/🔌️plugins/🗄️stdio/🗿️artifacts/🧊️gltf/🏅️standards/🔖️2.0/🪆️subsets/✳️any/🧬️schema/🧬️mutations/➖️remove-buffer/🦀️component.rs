//#region 🦠️Mutation
// 🦠️ `remove-buffer` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{remap_references, remove_checked, GltfMutationRejection, GltfSemanticMutation, IndexFamily};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveBuffer {
    pub index: usize,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for RemoveBuffer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "buffer", kind: "remove-buffer", record: "RemoveBuffer" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "RemoveBuffer".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for RemoveBuffer {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        let frozen = snapshot.document.clone();
        remove_checked(&mut snapshot.document.buffers, IndexFamily::Buffer, self.index, &frozen, "document/buffers")?;
        snapshot.buffers.remove(self.index);
        remap_references(&mut snapshot.document, IndexFamily::Buffer, self.index, false);
        Ok(())
    }
}
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `remove-buffer` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::RemoveBuffer;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn diff(payload: &RemoveBuffer, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `RemoveBuffer` semantic inverse.
    
    use super::RemoveBuffer;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn inverse(payload: &RemoveBuffer, base: &GltfSnapshot) -> Vec<GltfMutation> {
        match (base.document.buffers.get(payload.index), base.buffers.get(payload.index)) {
            (Some(buffer), Some(bytes)) => vec![GltfMutation::InsertBuffer(InsertBuffer { index: payload.index, buffer: buffer.clone(), bytes: bytes.clone() })],
            _ => Vec::new(),
        }
    }
}
//#endregion ↩️Inverse


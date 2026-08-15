//! 🦠️ `set-buffer` GLTF mutation payload.

use super::super::planning::{check_index, GltfMutationRejection, GltfSemanticMutation};
use crate::artifacts::gltf::schema::mutations::GltfMutation;
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
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        super::inverse::inverse(self, base)
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

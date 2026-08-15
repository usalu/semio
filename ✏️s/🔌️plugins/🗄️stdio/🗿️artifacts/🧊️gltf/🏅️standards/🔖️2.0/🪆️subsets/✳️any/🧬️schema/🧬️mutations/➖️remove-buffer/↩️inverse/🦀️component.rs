//! ↩️ `RemoveBuffer` semantic inverse.

use super::mutation::RemoveBuffer;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &RemoveBuffer, base: &GltfSnapshot) -> Vec<GltfMutation> {
    match (base.document.buffers.get(payload.index), base.buffers.get(payload.index)) {
        (Some(buffer), Some(bytes)) => vec![GltfMutation::InsertBuffer(InsertBuffer { index: payload.index, buffer: buffer.clone(), bytes: bytes.clone() })],
        _ => Vec::new(),
    }
}

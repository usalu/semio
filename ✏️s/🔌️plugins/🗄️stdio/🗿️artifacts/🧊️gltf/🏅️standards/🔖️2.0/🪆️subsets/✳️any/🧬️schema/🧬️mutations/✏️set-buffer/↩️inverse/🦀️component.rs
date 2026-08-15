//! ↩️ `SetBuffer` semantic inverse.

use super::mutation::SetBuffer;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &SetBuffer, base: &GltfSnapshot) -> Vec<GltfMutation> {
    match (base.document.buffers.get(payload.index), base.buffers.get(payload.index)) {
        (Some(buffer), Some(bytes)) => vec![GltfMutation::SetBuffer(SetBuffer { index: payload.index, buffer: buffer.clone(), bytes: bytes.clone() })],
        _ => Vec::new(),
    }
}

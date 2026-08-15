//! ↩️ `InsertBuffer` semantic inverse.

use super::mutation::InsertBuffer;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &InsertBuffer, _base: &GltfSnapshot) -> Vec<GltfMutation> {
    vec![GltfMutation::RemoveBuffer(RemoveBuffer { index: payload.index })]
}

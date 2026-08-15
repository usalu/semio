//! ↩️ `InsertAccessor` semantic inverse.

use super::mutation::InsertAccessor;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &InsertAccessor, _base: &GltfSnapshot) -> Vec<GltfMutation> {
    vec![GltfMutation::RemoveAccessor(RemoveAccessor { index: payload.index })]
}

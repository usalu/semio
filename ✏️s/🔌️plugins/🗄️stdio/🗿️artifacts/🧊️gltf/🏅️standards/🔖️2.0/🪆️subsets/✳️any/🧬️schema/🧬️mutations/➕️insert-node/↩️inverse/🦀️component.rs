//! ↩️ `InsertNode` semantic inverse.

use super::mutation::InsertNode;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &InsertNode, _base: &GltfSnapshot) -> Vec<GltfMutation> {
    vec![GltfMutation::RemoveNode(RemoveNode { index: payload.index })]
}

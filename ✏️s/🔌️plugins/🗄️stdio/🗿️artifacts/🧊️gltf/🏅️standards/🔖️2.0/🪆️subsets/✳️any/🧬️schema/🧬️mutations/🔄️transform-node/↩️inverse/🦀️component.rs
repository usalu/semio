//! ↩️ `TransformNode` semantic inverse.

use super::mutation::TransformNode;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &TransformNode, base: &GltfSnapshot) -> Vec<GltfMutation> {
    base.document.nodes.get(payload.index).map(|node| vec![GltfMutation::TransformNode(TransformNode { index: payload.index, matrix: node.matrix, translation: node.translation, rotation: node.rotation, scale: node.scale })]).unwrap_or_default()
}

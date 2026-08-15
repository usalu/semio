//! ↩️ `RemoveNode` semantic inverse.

use super::mutation::RemoveNode;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &RemoveNode, base: &GltfSnapshot) -> Vec<GltfMutation> {
    base.document.nodes.get(payload.index).map(|node| vec![GltfMutation::InsertNode(InsertNode { index: payload.index, node: node.clone() })]).unwrap_or_default()
}

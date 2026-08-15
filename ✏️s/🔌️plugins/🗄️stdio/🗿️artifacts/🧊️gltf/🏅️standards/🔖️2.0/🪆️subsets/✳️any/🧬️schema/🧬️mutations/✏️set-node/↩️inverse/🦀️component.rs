//! ↩️ `SetNode` semantic inverse.

use super::mutation::SetNode;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &SetNode, base: &GltfSnapshot) -> Vec<GltfMutation> {
    base.document.nodes.get(payload.index).map(|node| vec![GltfMutation::SetNode(SetNode { index: payload.index, node: node.clone() })]).unwrap_or_default()
}

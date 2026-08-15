//! ↩️ `ReparentNode` semantic inverse.

use super::mutation::ReparentNode;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &ReparentNode, base: &GltfSnapshot) -> Vec<GltfMutation> {
    locate_node_owner(&base.document, payload.index).map(|(parent, scene, position)| vec![GltfMutation::ReparentNode(ReparentNode { index: payload.index, parent, scene, position })]).unwrap_or_default()
}

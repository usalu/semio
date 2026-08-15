//! ↩️ `SetAsset` semantic inverse.

use super::mutation::SetAsset;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(_payload: &SetAsset, base: &GltfSnapshot) -> Vec<GltfMutation> {
    vec![GltfMutation::SetAsset(SetAsset { asset: base.document.asset.clone() })]
}

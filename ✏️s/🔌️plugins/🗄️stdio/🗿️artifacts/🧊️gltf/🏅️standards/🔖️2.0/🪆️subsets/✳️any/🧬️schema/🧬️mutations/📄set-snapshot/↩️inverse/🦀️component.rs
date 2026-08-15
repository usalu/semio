//! ↩️ `set-snapshot` semantic inverse.

use super::mutation::SetSnapshot;
use crate::artifacts::gltf::schema::mutations::GltfMutation;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(_payload: &SetSnapshot, base: &GltfSnapshot) -> Vec<GltfMutation> {
    vec![GltfMutation::SetSnapshot(SetSnapshot { snapshot: base.clone() })]
}

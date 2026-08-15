//! ↩️ `InsertAnimation` semantic inverse.

use super::mutation::InsertAnimation;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &InsertAnimation, _base: &GltfSnapshot) -> Vec<GltfMutation> {
    vec![GltfMutation::RemoveAnimation(RemoveAnimation { index: payload.index })]
}

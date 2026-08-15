//! ↩️ `NoMutation` semantic inverse.

use super::mutation::NoMutation;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(_payload: &NoMutation, _base: &GltfSnapshot) -> Vec<GltfMutation> {
    Vec::new()
}

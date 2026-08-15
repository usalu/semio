//! ↩️ `InsertScene` semantic inverse.

use super::mutation::InsertScene;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &InsertScene, _base: &GltfSnapshot) -> Vec<GltfMutation> {
    vec![GltfMutation::RemoveScene(RemoveScene { index: payload.index })]
}

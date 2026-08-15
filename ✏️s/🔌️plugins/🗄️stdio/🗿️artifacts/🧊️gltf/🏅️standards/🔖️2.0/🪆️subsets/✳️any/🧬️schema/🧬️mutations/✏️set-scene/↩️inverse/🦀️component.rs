//! ↩️ `SetScene` semantic inverse.

use super::mutation::SetScene;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &SetScene, base: &GltfSnapshot) -> Vec<GltfMutation> {
    base.document.scenes.get(payload.index).map(|scene| vec![GltfMutation::SetScene(SetScene { index: payload.index, scene: scene.clone() })]).unwrap_or_default()
}

//! ↩️ `RemoveScene` semantic inverse.

use super::mutation::RemoveScene;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &RemoveScene, base: &GltfSnapshot) -> Vec<GltfMutation> {
    base.document.scenes.get(payload.index).map(|scene| vec![GltfMutation::InsertScene(InsertScene { index: payload.index, scene: scene.clone() })]).unwrap_or_default()
}

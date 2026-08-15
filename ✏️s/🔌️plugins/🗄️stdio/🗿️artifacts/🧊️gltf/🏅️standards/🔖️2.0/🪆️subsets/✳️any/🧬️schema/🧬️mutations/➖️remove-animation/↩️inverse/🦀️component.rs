//! ↩️ `RemoveAnimation` semantic inverse.

use super::mutation::RemoveAnimation;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &RemoveAnimation, base: &GltfSnapshot) -> Vec<GltfMutation> {
    base.document.animations.get(payload.index).map(|animation| vec![GltfMutation::InsertAnimation(InsertAnimation { index: payload.index, animation: animation.clone() })]).unwrap_or_default()
}

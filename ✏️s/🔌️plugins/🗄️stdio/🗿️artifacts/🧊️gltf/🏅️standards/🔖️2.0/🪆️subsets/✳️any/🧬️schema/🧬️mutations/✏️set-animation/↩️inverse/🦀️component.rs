//! ↩️ `SetAnimation` semantic inverse.

use super::mutation::SetAnimation;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &SetAnimation, base: &GltfSnapshot) -> Vec<GltfMutation> {
    base.document.animations.get(payload.index).map(|animation| vec![GltfMutation::SetAnimation(SetAnimation { index: payload.index, animation: animation.clone() })]).unwrap_or_default()
}

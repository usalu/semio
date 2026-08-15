//! 🔺️ `remove-animation` validated sparse diff.

use super::super::planning::GltfSemanticMutation;
use super::mutation::RemoveAnimation;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &RemoveAnimation, base: &GltfSnapshot) -> GltfDiff {
    payload.plan(base).unwrap_or_default()
}

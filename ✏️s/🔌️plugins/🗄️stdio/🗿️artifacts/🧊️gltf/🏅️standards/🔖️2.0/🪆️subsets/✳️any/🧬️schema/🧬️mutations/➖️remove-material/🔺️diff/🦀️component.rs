//! 🔺️ `remove-material` validated sparse diff.

use super::super::planning::GltfSemanticMutation;
use super::mutation::RemoveMaterial;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &RemoveMaterial, base: &GltfSnapshot) -> GltfDiff {
    payload.plan(base).unwrap_or_default()
}

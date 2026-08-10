use crate::artifacts::gltf::{GltfSnapshot};
use crate::artifacts::gltf::schema::mutations::{GltfMutation, apply_gltf_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut GltfSnapshot, mutation: &GltfMutation) {
    apply_gltf_mutation(projection, mutation);
}

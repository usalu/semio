use crate::artifacts::gltf::schema::mutations::{apply_gltf_mutation, GltfMutation};
use crate::artifacts::gltf::GltfSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut GltfSnapshot, mutation: &GltfMutation) {
    apply_gltf_mutation(projection, mutation);
}

use crate::artifacts::gltf::schema::mutations::{apply_gltf_mutation, GltfMutation, GltfMutationRejection};
use crate::artifacts::gltf::{GltfDiff, GltfSnapshot};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut GltfSnapshot, mutation: &GltfMutation) -> Result<GltfDiff, GltfMutationRejection> {
    apply_gltf_mutation(projection, mutation)
}

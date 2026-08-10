use crate::artifacts::gltf::{GltfSnapshot};
use crate::artifacts::gltf::schema::mutations::GltfMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &GltfSnapshot, mutation: &GltfMutation) -> Vec<GltfMutation> {
    <GltfMutation as Mutation<GltfSnapshot>>::inverse(mutation, base)
}

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::{SemioMeshMutation, apply_semio_mesh_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SemioMeshSnapshot, mutation: &SemioMeshMutation) {
    let _ = apply_semio_mesh_mutation(projection, mutation);
}

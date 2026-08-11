use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &SemioMeshSnapshot, mutation: &SemioMeshMutation) -> Vec<SemioMeshMutation> {
    <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::inverse(mutation, base)
}

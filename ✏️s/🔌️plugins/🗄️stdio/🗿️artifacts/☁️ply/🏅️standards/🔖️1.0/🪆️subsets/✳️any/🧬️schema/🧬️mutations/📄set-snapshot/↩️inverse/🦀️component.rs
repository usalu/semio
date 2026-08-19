use crate::artifacts::ply::schema::mutations::PlyMutation;
use crate::artifacts::ply::PlySnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &PlySnapshot, mutation: &PlyMutation) -> Vec<PlyMutation> {
    <PlyMutation as Mutation<PlySnapshot>>::inverse(mutation, base)
}

use crate::artifacts::ply::{PlySnapshot};
use crate::artifacts::ply::schema::mutations::PlyMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &PlySnapshot, mutation: &PlyMutation) -> Vec<PlyMutation> {
    <PlyMutation as Mutation<PlySnapshot>>::inverse(mutation, base)
}

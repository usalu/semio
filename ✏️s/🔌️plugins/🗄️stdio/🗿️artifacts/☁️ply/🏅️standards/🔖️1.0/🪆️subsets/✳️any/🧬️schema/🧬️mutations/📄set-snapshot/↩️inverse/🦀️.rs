use crate::artifacts::ply::schema::mutations::PlyMutation;
use crate::artifacts::ply::PlySnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &PlySnapshot, mutation: &PlyMutation) -> Vec<PlyMutation> {
    <PlyMutation as Mutation<PlySnapshot>>::inverse(mutation, base)
}

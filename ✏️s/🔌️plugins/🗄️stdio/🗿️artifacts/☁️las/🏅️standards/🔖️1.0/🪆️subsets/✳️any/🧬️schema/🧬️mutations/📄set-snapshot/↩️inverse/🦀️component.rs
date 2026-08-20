use crate::artifacts::las::schema::mutations::LasMutation;
use crate::artifacts::las::LasSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &LasSnapshot, mutation: &LasMutation) -> Vec<LasMutation> {
    <LasMutation as Mutation<LasSnapshot>>::inverse(mutation, base)
}

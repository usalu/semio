use crate::artifacts::stl::schema::mutations::StlMutation;
use crate::artifacts::stl::StlSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &StlSnapshot, mutation: &StlMutation) -> Vec<StlMutation> {
    <StlMutation as Mutation<StlSnapshot>>::inverse(mutation, base)
}

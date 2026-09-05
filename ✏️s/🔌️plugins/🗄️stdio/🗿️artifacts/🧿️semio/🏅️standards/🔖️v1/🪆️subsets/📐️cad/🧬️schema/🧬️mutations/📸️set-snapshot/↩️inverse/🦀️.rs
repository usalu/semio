use crate::artifacts::semio::standards::v1::subsets::cad::schema::mutations::SemioCadMutation;
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &SemioCadSnapshot, mutation: &SemioCadMutation) -> Vec<SemioCadMutation> {
    <SemioCadMutation as Mutation<SemioCadSnapshot>>::inverse(mutation, base)
}

use crate::artifacts::semio::standards::v1::subsets::presentation::schema::mutations::SemioPresentationMutation;
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &SemioPresentationSnapshot, mutation: &SemioPresentationMutation) -> Vec<SemioPresentationMutation> {
    <SemioPresentationMutation as Mutation<SemioPresentationSnapshot>>::inverse(mutation, base)
}

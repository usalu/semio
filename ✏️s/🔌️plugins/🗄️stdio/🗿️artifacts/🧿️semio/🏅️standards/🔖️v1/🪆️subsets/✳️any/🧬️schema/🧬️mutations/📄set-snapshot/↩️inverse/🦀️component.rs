use crate::artifacts::semio::standards::v1::subsets::any::schema::mutations::SemioMutation;
use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::SemioSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &SemioSnapshot, mutation: &SemioMutation) -> Vec<SemioMutation> {
    <SemioMutation as Mutation<SemioSnapshot>>::inverse(mutation, base)
}

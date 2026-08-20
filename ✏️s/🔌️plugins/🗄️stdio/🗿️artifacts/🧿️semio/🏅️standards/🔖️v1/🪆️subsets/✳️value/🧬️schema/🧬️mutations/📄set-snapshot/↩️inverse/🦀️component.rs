use crate::artifacts::semio::standards::v1::subsets::value::schema::mutations::SemioValueMutation;
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &SemioValueSnapshot, mutation: &SemioValueMutation) -> Vec<SemioValueMutation> {
    <SemioValueMutation as Mutation<SemioValueSnapshot>>::inverse(mutation, base)
}

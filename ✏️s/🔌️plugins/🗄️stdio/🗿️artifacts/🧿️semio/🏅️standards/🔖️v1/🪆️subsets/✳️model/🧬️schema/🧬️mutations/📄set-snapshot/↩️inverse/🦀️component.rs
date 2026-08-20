use crate::artifacts::semio::standards::v1::subsets::model::schema::mutations::SemioModelMutation;
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &SemioModelSnapshot, mutation: &SemioModelMutation) -> Vec<SemioModelMutation> {
    <SemioModelMutation as Mutation<SemioModelSnapshot>>::inverse(mutation, base)
}

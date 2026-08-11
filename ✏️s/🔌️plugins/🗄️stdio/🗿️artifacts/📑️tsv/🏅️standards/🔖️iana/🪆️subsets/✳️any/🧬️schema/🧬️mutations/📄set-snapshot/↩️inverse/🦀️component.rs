use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::TsvSnapshot;
use crate::artifacts::tsv::standards::iana::subsets::any::schema::mutations::TsvMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &TsvSnapshot, mutation: &TsvMutation) -> Vec<TsvMutation> {
    <TsvMutation as Mutation<TsvSnapshot>>::inverse(mutation, base)
}

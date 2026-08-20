use crate::artifacts::tsv::standards::iana::subsets::any::schema::mutations::TsvMutation;
use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::TsvSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &TsvSnapshot, mutation: &TsvMutation) -> Vec<TsvMutation> {
    <TsvMutation as Mutation<TsvSnapshot>>::inverse(mutation, base).await
}

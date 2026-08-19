use crate::artifacts::semio::standards::v1::subsets::any::schema::mutations::SemioMutation;
use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::SemioSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &SemioSnapshot, mutation: &SemioMutation) -> Vec<SemioMutation> {
    <SemioMutation as Mutation<SemioSnapshot>>::inverse(mutation, base)
}

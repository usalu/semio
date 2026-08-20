use crate::artifacts::semio::standards::v1::subsets::value::schema::mutations::SemioValueMutation;
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &SemioValueSnapshot, mutation: &SemioValueMutation) -> Vec<SemioValueMutation> {
    <SemioValueMutation as Mutation<SemioValueSnapshot>>::inverse(mutation, base).await
}

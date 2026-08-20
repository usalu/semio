use crate::artifacts::semio::standards::v1::subsets::cad::schema::mutations::SemioCadMutation;
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &SemioCadSnapshot, mutation: &SemioCadMutation) -> Vec<SemioCadMutation> {
    <SemioCadMutation as Mutation<SemioCadSnapshot>>::inverse(mutation, base).await
}

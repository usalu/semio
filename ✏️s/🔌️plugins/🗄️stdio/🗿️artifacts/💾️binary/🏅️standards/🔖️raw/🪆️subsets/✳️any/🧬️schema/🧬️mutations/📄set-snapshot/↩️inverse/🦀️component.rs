use crate::artifacts::binary::schema::mutations::BinaryMutation;
use crate::artifacts::binary::BinarySnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &BinarySnapshot, mutation: &BinaryMutation) -> Vec<BinaryMutation> {
    <BinaryMutation as Mutation<BinarySnapshot>>::inverse(mutation, base).await
}

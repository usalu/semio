use crate::artifacts::jpg::schema::mutations::JpgMutation;
use crate::artifacts::jpg::JpgSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &JpgSnapshot, mutation: &JpgMutation) -> Vec<JpgMutation> {
    <JpgMutation as Mutation<JpgSnapshot>>::inverse(mutation, base).await
}

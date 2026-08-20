use crate::artifacts::md::schema::mutations::MdMutation;
use crate::artifacts::md::MdSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &MdSnapshot, mutation: &MdMutation) -> Vec<MdMutation> {
    <MdMutation as Mutation<MdSnapshot>>::inverse(mutation, base).await
}

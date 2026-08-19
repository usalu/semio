use crate::artifacts::bcf::schema::mutations::BcfMutation;
use crate::artifacts::bcf::BcfSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &BcfSnapshot, mutation: &BcfMutation) -> Vec<BcfMutation> {
    <BcfMutation as Mutation<BcfSnapshot>>::inverse(mutation, base)
}

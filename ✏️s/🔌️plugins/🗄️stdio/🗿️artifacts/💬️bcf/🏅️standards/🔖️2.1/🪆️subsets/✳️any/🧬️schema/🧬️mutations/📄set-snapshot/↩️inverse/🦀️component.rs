use crate::artifacts::bcf::{BcfSnapshot};
use crate::artifacts::bcf::schema::mutations::BcfMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &BcfSnapshot, mutation: &BcfMutation) -> Vec<BcfMutation> {
    <BcfMutation as Mutation<BcfSnapshot>>::inverse(mutation, base)
}

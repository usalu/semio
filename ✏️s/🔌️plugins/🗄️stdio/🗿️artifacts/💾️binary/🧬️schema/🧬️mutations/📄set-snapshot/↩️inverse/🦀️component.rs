use crate::artifacts::binary::{BinarySnapshot};
use crate::artifacts::binary::schema::mutations::BinaryMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &BinarySnapshot, mutation: &BinaryMutation) -> Vec<BinaryMutation> {
    <BinaryMutation as Mutation<BinarySnapshot>>::inverse(mutation, base)
}

use crate::artifacts::md::{MdSnapshot};
use crate::artifacts::md::schema::mutations::MdMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &MdSnapshot, mutation: &MdMutation) -> Vec<MdMutation> {
    <MdMutation as Mutation<MdSnapshot>>::inverse(mutation, base)
}

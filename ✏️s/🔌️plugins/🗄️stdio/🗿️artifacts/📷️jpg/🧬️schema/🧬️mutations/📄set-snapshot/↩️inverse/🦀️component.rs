use crate::artifacts::jpg::{JpgSnapshot};
use crate::artifacts::jpg::schema::mutations::JpgMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &JpgSnapshot, mutation: &JpgMutation) -> Vec<JpgMutation> {
    <JpgMutation as Mutation<JpgSnapshot>>::inverse(mutation, base)
}

use crate::artifacts::binary::schema::mutations::{apply_binary_mutation, BinaryMutation};
use crate::artifacts::binary::BinarySnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut BinarySnapshot, mutation: &BinaryMutation) {
    apply_binary_mutation(projection, mutation);
}

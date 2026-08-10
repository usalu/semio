use crate::artifacts::binary::{BinarySnapshot};
use crate::artifacts::binary::schema::mutations::{BinaryMutation, apply_binary_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut BinarySnapshot, mutation: &BinaryMutation) {
    apply_binary_mutation(projection, mutation);
}

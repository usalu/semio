use crate::artifacts::binary::schema::mutations::{apply_binary_mutation, BinaryMutation};
use crate::artifacts::binary::BinarySnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut BinarySnapshot, mutation: &BinaryMutation) {
    apply_binary_mutation(projection, mutation);
}

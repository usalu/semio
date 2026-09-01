//! ↩️ Inverse for `set-snapshot`.

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::binary::schema::mutations::{BinaryMutation, apply_binary_mutation};
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &BinarySnapshot, mutation: &BinaryMutation) -> Vec<BinaryMutation> {
    <BinaryMutation as Mutation<BinarySnapshot>>::inverse(mutation, base)
}

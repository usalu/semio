//! ↩️ Inverse for `set-snapshot`.

use crate::artifacts::deflate::DeflateSnapshot;
use crate::artifacts::deflate::schema::mutations::{DeflateMutation, apply_deflate_mutation};
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &DeflateSnapshot, mutation: &DeflateMutation) -> Vec<DeflateMutation> {
    <DeflateMutation as Mutation<DeflateSnapshot>>::inverse(mutation, base)
}

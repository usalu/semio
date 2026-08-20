use crate::artifacts::deflate::schema::mutations::{apply_deflate_mutation, DeflateMutation};
use crate::artifacts::deflate::DeflateSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut DeflateSnapshot, mutation: &DeflateMutation) {
    apply_deflate_mutation(projection, mutation);
}

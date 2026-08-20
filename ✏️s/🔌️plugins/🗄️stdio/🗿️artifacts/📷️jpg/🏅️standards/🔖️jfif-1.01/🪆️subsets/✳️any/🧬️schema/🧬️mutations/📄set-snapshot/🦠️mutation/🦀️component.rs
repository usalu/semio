use crate::artifacts::jpg::schema::mutations::{apply_jpg_mutation, JpgMutation};
use crate::artifacts::jpg::JpgSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut JpgSnapshot, mutation: &JpgMutation) {
    apply_jpg_mutation(projection, mutation);
}

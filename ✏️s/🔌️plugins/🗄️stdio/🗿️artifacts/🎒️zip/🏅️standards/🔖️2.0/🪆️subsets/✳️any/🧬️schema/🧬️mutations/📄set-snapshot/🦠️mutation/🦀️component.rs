use crate::artifacts::zip::schema::mutations::{apply_zip_mutation, ZipMutation};
use crate::artifacts::zip::ZipSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut ZipSnapshot, mutation: &ZipMutation) {
    apply_zip_mutation(projection, mutation);
}

use crate::artifacts::tiff::schema::mutations::{apply_tiff_mutation, TiffMutation};
use crate::artifacts::tiff::TiffSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut TiffSnapshot, mutation: &TiffMutation) {
    apply_tiff_mutation(projection, mutation);
}

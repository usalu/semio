use crate::artifacts::png::schema::mutations::{apply_png_mutation, PngMutation};
use crate::artifacts::png::PngSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut PngSnapshot, mutation: &PngMutation) {
    apply_png_mutation(projection, mutation);
}

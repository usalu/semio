use crate::artifacts::png::schema::mutations::{apply_png_mutation, PngMutation};
use crate::artifacts::png::PngSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut PngSnapshot, mutation: &PngMutation) {
    apply_png_mutation(projection, mutation);
}

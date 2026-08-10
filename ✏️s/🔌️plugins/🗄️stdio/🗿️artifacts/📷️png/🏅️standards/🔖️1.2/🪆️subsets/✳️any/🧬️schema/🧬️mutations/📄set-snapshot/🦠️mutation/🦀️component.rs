use crate::artifacts::png::{PngSnapshot};
use crate::artifacts::png::schema::mutations::{PngMutation, apply_png_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut PngSnapshot, mutation: &PngMutation) {
    apply_png_mutation(projection, mutation);
}

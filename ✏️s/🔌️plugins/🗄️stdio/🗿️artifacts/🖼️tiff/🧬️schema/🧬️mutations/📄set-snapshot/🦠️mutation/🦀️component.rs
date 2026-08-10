use crate::artifacts::tiff::{TiffSnapshot};
use crate::artifacts::tiff::schema::mutations::{TiffMutation, apply_tiff_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut TiffSnapshot, mutation: &TiffMutation) {
    apply_tiff_mutation(projection, mutation);
}

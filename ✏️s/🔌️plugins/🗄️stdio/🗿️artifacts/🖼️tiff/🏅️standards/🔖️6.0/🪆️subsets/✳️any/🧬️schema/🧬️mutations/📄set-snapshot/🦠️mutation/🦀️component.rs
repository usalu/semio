use crate::artifacts::tiff::schema::mutations::{apply_tiff_mutation, TiffMutation};
use crate::artifacts::tiff::TiffSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut TiffSnapshot, mutation: &TiffMutation) {
    apply_tiff_mutation(projection, mutation);
}

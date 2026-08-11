use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{SemioImageMutation, apply_semio_image_mutation};

/// ▶️ Applies a set-bit-depth mutation.
pub fn apply(snapshot: &mut SemioImageSnapshot, bit_depth: u8) -> SemioImageDiff {
    apply_semio_image_mutation(snapshot, &SemioImageMutation::SetBitDepth { bit_depth })
}

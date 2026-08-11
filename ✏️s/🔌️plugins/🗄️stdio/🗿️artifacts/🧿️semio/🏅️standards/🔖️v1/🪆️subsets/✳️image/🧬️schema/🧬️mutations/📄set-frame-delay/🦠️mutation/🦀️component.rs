use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{SemioImageMutation, apply_semio_image_mutation};

/// ▶️ Applies a set-frame-delay mutation.
pub fn apply(snapshot: &mut SemioImageSnapshot, index: usize, delay_ms: u32) -> SemioImageDiff {
    apply_semio_image_mutation(snapshot, &SemioImageMutation::SetFrameDelay { index, delay_ms })
}

use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{SemioImageMutation, apply_semio_image_mutation};

/// ▶️ Applies a set-frame-pixels mutation.
pub async fn apply(snapshot: &mut SemioImageSnapshot, index: usize, rgba8: Vec<u8>) -> SemioImageDiff {
    apply_semio_image_mutation(snapshot, &SemioImageMutation::SetFramePixels { index, rgba8 })
}

use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{SemioImageMutation, apply_semio_image_mutation};

/// ▶️ Applies a move-frame mutation.
pub async fn apply(snapshot: &mut SemioImageSnapshot, from: usize, to: usize) -> SemioImageDiff {
    apply_semio_image_mutation(snapshot, &SemioImageMutation::MoveFrame { from, to })
}

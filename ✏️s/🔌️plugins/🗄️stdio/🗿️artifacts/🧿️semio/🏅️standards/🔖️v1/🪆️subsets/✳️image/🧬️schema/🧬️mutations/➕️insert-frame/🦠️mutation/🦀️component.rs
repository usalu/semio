use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioImageFrame, SemioImageSnapshot};
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{SemioImageMutation, apply_semio_image_mutation};

/// ▶️ Applies an insert-frame mutation.
pub async fn apply(snapshot: &mut SemioImageSnapshot, index: usize, frame: SemioImageFrame) -> SemioImageDiff {
    apply_semio_image_mutation(snapshot, &SemioImageMutation::InsertFrame { index, frame })
}

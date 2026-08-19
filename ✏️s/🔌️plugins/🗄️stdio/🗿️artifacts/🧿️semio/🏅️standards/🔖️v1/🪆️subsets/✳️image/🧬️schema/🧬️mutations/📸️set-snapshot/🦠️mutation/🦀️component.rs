use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{apply_semio_image_mutation, SemioImageMutation};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut SemioImageSnapshot, mutation: &SemioImageMutation) {
    let _ = apply_semio_image_mutation(projection, mutation);
}

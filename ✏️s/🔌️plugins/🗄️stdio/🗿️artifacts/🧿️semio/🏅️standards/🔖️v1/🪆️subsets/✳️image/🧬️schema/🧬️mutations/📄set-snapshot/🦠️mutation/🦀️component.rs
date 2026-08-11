use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{SemioImageMutation, apply_semio_image_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SemioImageSnapshot, mutation: &SemioImageMutation) {
    let _ = apply_semio_image_mutation(projection, mutation);
}

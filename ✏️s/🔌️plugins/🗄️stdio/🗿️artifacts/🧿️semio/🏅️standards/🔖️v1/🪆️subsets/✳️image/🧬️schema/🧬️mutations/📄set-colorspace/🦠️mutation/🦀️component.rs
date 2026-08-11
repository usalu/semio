use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageSnapshot};
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{SemioImageMutation, apply_semio_image_mutation};

/// ▶️ Applies a set-colorspace mutation.
pub fn apply(snapshot: &mut SemioImageSnapshot, colorspace: SemioColorspace) -> SemioImageDiff {
    apply_semio_image_mutation(snapshot, &SemioImageMutation::SetColorspace { colorspace })
}

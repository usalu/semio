use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{SemioImageMutation, apply_semio_image_mutation};

/// ▶️ Applies a set-icc mutation (`icc: None` clears the embedded profile).
pub fn apply(snapshot: &mut SemioImageSnapshot, icc: Option<Vec<u8>>) -> SemioImageDiff {
    apply_semio_image_mutation(snapshot, &SemioImageMutation::SetIcc { icc })
}

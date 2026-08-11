use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{SemioImageMutation, apply_semio_image_mutation};

/// ▶️ Applies a set-metadata-entry mutation (inserts or updates, keyed by `key`).
pub fn apply(snapshot: &mut SemioImageSnapshot, key: String, value: String) -> SemioImageDiff {
    apply_semio_image_mutation(snapshot, &SemioImageMutation::SetMetadataEntry { key, value })
}

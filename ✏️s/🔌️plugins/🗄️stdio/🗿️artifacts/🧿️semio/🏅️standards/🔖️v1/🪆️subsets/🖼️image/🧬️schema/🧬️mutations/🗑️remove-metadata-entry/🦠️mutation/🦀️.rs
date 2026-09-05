use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{SemioImageMutation, apply_semio_image_mutation};
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::remove_metadata_entry;

/// ▶️ Applies a remove-metadata-entry mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(snapshot: &mut SemioImageSnapshot, key: String) -> SemioImageDiff {
    apply_semio_image_mutation(snapshot, &SemioImageMutation::RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry { key }))
}

use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// 🔺️ Diff helper for remove-metadata-entry.
pub fn diff(base: &SemioImageSnapshot, key: String) -> SemioImageDiff {
    Mutation::diff(&SemioImageMutation::RemoveMetadataEntry { key }, base)
}

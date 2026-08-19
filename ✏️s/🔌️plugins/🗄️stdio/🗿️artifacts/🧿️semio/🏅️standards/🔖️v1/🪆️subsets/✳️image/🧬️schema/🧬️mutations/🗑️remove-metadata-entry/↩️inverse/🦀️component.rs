use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// ↩️ Inverse of remove-metadata-entry.
pub async fn inverse(base: &SemioImageSnapshot, key: String) -> Vec<SemioImageMutation> {
    <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&SemioImageMutation::RemoveMetadataEntry { key }, base)
}

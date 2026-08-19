use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// 🔺️ Diff helper for remove-metadata-entry — a `key` absent from `base.metadata` is
/// `mutation.target-missing` (Error, empty diff).
pub async fn diff(base: &SemioImageSnapshot, key: String) -> protocol::MutationOutcome<SemioImageDiff> {
    if !base.metadata.iter().any(|e| e.key == key) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Metadata entry \"{key}\" does not exist."), [key.clone()]);
    }
    Mutation::diff(&SemioImageMutation::RemoveMetadataEntry { key }, base)
}

use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// 🔺️ Diff helper for set-metadata-entry — an upsert (adds the entry when `key` is absent from
/// `base.metadata`, otherwise updates its value), so there is no "target missing" case. An
/// existing entry already holding this exact `value` is `mutation.no-op` (Warning, empty diff).
pub async fn diff(base: &SemioImageSnapshot, key: String, value: String) -> protocol::MutationOutcome<SemioImageDiff> {
    if base.metadata.iter().any(|e| e.key == key && e.value == value) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Metadata entry \"{key}\" already has this value."));
    }
    Mutation::diff(&SemioImageMutation::SetMetadataEntry { key, value }, base)
}

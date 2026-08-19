use crate::artifacts::semio::standards::v1::subsets::any::schema::mutations::{apply_semio_mutation, SemioMutation};
use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::SemioSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut SemioSnapshot, mutation: &SemioMutation) {
    let _ = apply_semio_mutation(projection, mutation);
}

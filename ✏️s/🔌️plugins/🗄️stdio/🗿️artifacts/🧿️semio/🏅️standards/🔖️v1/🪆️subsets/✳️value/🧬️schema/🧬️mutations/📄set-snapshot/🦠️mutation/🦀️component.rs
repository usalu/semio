use crate::artifacts::semio::standards::v1::subsets::value::schema::mutations::{apply_semio_value_mutation, SemioValueMutation};
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut SemioValueSnapshot, mutation: &SemioValueMutation) {
    let _ = apply_semio_value_mutation(projection, mutation);
}

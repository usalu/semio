use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use crate::artifacts::semio::standards::v1::subsets::value::schema::mutations::{SemioValueMutation, apply_semio_value_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SemioValueSnapshot, mutation: &SemioValueMutation) {
    let _ = apply_semio_value_mutation(projection, mutation);
}

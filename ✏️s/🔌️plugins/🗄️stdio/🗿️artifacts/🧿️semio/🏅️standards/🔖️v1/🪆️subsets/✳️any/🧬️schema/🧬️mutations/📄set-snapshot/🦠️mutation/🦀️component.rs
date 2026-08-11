use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::SemioSnapshot;
use crate::artifacts::semio::standards::v1::subsets::any::schema::mutations::{SemioMutation, apply_semio_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SemioSnapshot, mutation: &SemioMutation) {
    let _ = apply_semio_mutation(projection, mutation);
}

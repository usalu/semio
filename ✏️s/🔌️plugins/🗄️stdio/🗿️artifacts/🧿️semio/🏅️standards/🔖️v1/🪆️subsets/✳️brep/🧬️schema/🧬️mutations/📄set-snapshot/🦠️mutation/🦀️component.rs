use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{SemioBrepMutation, apply_semio_brep_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SemioBrepSnapshot, mutation: &SemioBrepMutation) {
    let _ = apply_semio_brep_mutation(projection, mutation);
}

use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;
use crate::artifacts::semio::standards::v1::subsets::cad::schema::mutations::{SemioCadMutation, apply_semio_cad_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SemioCadSnapshot, mutation: &SemioCadMutation) {
    let _ = apply_semio_cad_mutation(projection, mutation);
}

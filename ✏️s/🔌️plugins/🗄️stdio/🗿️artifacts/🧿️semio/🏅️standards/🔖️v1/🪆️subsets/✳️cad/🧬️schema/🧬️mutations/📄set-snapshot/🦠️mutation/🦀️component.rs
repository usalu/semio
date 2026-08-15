use crate::artifacts::semio::standards::v1::subsets::cad::schema::mutations::{apply_semio_cad_mutation, SemioCadMutation};
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SemioCadSnapshot, mutation: &SemioCadMutation) {
    let _ = apply_semio_cad_mutation(projection, mutation);
}

use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::mutations::{SemioPresentationMutation, apply_semio_presentation_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SemioPresentationSnapshot, mutation: &SemioPresentationMutation) {
    let _ = apply_semio_presentation_mutation(projection, mutation);
}

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{SemioDrawingMutation, apply_semio_drawing_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SemioDrawingSnapshot, mutation: &SemioDrawingMutation) {
    let _ = apply_semio_drawing_mutation(projection, mutation);
}

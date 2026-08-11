use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use crate::artifacts::semio::standards::v1::subsets::animation::schema::mutations::{SemioAnimationMutation, apply_semio_animation_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SemioAnimationSnapshot, mutation: &SemioAnimationMutation) {
    let _ = apply_semio_animation_mutation(projection, mutation);
}

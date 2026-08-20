use crate::artifacts::semio::standards::v1::subsets::animation::schema::mutations::{apply_semio_animation_mutation, SemioAnimationMutation};
use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut SemioAnimationSnapshot, mutation: &SemioAnimationMutation) {
    let _ = apply_semio_animation_mutation(projection, mutation);
}

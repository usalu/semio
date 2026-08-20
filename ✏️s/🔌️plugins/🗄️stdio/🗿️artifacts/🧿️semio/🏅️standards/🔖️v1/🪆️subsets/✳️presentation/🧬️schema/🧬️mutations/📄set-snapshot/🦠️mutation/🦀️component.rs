use crate::artifacts::semio::standards::v1::subsets::presentation::schema::mutations::{apply_semio_presentation_mutation, SemioPresentationMutation};
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut SemioPresentationSnapshot, mutation: &SemioPresentationMutation) {
    let _ = apply_semio_presentation_mutation(projection, mutation);
}

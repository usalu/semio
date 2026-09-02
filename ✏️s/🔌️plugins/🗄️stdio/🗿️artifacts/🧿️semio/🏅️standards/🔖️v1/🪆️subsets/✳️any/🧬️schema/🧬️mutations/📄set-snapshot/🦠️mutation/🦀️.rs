use crate::artifacts::semio::standards::v1::subsets::any::schema::mutations::{apply_semio_mutation, SemioMutation};
use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::SemioSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut SemioSnapshot, mutation: &SemioMutation) {
    let _ = apply_semio_mutation(projection, mutation);
}

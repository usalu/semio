use crate::artifacts::semio::standards::v1::subsets::cad::schema::mutations::{apply_semio_cad_mutation, SemioCadMutation};
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut SemioCadSnapshot, mutation: &SemioCadMutation) {
    let _ = apply_semio_cad_mutation(projection, mutation);
}

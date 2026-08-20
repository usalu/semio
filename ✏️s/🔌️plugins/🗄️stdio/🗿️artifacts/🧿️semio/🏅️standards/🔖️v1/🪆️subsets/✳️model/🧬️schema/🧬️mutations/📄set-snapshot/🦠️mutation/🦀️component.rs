use crate::artifacts::semio::standards::v1::subsets::model::schema::mutations::{apply_semio_model_mutation, SemioModelMutation};
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut SemioModelSnapshot, mutation: &SemioModelMutation) {
    let _ = apply_semio_model_mutation(projection, mutation);
}

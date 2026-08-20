use crate::artifacts::las::schema::mutations::{apply_las_mutation, LasMutation};
use crate::artifacts::las::{LasDiff, LasSnapshot};

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut LasSnapshot, mutation: &LasMutation) -> protocol::MutationOutcome<LasDiff> {
    apply_las_mutation(projection, mutation).await
}

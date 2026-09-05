use crate::artifacts::ply::schema::mutations::{apply_ply_mutation, PlyMutation};
use crate::artifacts::ply::{PlyDiff, PlySnapshot};

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut PlySnapshot, mutation: &PlyMutation) -> protocol::MutationOutcome<PlyDiff> {
    apply_ply_mutation(projection, mutation)
}

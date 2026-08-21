use crate::artifacts::stl::schema::mutations::{apply_stl_mutation, StlMutation};
use crate::artifacts::stl::{StlDiff, StlSnapshot};

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut StlSnapshot, mutation: &StlMutation) -> protocol::MutationOutcome<StlDiff> {
    apply_stl_mutation(projection, mutation)
}

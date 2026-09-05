use crate::artifacts::dxf::schema::mutations::{apply_dxf_mutation, DxfMutation};
use crate::artifacts::dxf::{DxfDiff, DxfSnapshot};

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut DxfSnapshot, mutation: &DxfMutation) -> protocol::MutationOutcome<DxfDiff> {
    apply_dxf_mutation(projection, mutation)
}

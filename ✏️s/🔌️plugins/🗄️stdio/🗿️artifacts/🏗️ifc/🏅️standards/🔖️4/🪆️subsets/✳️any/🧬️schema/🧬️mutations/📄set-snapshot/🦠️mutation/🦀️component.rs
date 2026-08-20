use crate::artifacts::ifc::schema::mutations::{apply_ifc_mutation, IfcMutation};
use crate::artifacts::ifc::{IfcDiff, IfcSnapshot};

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut IfcSnapshot, mutation: &IfcMutation) -> protocol::MutationOutcome<IfcDiff> {
    apply_ifc_mutation(projection, mutation)
}

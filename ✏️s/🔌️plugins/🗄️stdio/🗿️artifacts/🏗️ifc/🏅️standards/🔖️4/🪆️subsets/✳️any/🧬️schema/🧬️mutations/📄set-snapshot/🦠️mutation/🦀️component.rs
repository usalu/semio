use crate::artifacts::ifc::schema::mutations::{apply_ifc_mutation, IfcMutation};
use crate::artifacts::ifc::IfcSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut IfcSnapshot, mutation: &IfcMutation) {
    apply_ifc_mutation(projection, mutation);
}

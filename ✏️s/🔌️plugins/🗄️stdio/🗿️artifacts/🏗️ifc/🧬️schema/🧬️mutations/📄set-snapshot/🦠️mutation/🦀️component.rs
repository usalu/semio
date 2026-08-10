use crate::artifacts::ifc::{IfcSnapshot};
use crate::artifacts::ifc::schema::mutations::{IfcMutation, apply_ifc_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut IfcSnapshot, mutation: &IfcMutation) {
    apply_ifc_mutation(projection, mutation);
}

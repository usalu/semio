use crate::artifacts::ifc::{IfcSnapshot};
use crate::artifacts::ifc::schema::mutations::IfcMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &IfcSnapshot, mutation: &IfcMutation) -> Vec<IfcMutation> {
    <IfcMutation as Mutation<IfcSnapshot>>::inverse(mutation, base)
}

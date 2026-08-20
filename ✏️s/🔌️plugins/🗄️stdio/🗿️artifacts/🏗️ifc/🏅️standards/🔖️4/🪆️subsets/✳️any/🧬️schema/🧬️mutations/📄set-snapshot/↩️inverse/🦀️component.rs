use crate::artifacts::ifc::schema::mutations::IfcMutation;
use crate::artifacts::ifc::IfcSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &IfcSnapshot, mutation: &IfcMutation) -> Vec<IfcMutation> {
    <IfcMutation as Mutation<IfcSnapshot>>::inverse(mutation, base).await
}

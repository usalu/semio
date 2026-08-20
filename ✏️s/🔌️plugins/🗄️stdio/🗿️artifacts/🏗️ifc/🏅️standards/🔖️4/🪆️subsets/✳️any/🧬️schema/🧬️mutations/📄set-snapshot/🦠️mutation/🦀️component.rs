use crate::artifacts::ifc::schema::mutations::{apply_ifc_mutation, IfcMutation};
use crate::artifacts::ifc::{IfcDiff, IfcSnapshot};

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut IfcSnapshot, mutation: &IfcMutation) -> protocol::MutationOutcome<IfcDiff> {
    apply_ifc_mutation(projection, mutation).await
}

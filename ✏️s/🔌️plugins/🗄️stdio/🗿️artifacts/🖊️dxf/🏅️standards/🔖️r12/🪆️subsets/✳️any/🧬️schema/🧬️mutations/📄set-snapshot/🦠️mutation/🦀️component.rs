use crate::artifacts::dxf::schema::mutations::{apply_dxf_mutation, DxfMutation};
use crate::artifacts::dxf::{DxfDiff, DxfSnapshot};

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut DxfSnapshot, mutation: &DxfMutation) -> protocol::MutationOutcome<DxfDiff> {
    apply_dxf_mutation(projection, mutation).await
}

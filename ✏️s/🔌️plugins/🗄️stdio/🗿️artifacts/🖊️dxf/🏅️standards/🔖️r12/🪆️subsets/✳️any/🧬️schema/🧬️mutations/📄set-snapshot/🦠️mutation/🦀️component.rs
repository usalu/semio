use crate::artifacts::dxf::schema::mutations::{apply_dxf_mutation, DxfMutation};
use crate::artifacts::dxf::DxfSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut DxfSnapshot, mutation: &DxfMutation) {
    apply_dxf_mutation(projection, mutation);
}

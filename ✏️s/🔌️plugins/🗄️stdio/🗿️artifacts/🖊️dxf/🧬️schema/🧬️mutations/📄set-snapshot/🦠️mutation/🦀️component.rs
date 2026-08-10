use crate::artifacts::dxf::{DxfSnapshot};
use crate::artifacts::dxf::schema::mutations::{DxfMutation, apply_dxf_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut DxfSnapshot, mutation: &DxfMutation) {
    apply_dxf_mutation(projection, mutation);
}

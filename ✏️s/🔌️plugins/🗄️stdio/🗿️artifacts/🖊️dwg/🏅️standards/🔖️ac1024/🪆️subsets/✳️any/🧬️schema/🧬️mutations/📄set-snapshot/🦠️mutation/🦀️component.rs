use crate::artifacts::dwg::schema::mutations::{apply_dwg_mutation, DwgMutation};
use crate::artifacts::dwg::DwgSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut DwgSnapshot, mutation: &DwgMutation) {
    apply_dwg_mutation(projection, mutation);
}

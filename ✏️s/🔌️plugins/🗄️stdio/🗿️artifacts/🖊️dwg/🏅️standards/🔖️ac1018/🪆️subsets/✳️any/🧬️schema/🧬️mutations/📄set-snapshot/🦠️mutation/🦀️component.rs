use crate::artifacts::dwg::{DwgSnapshot};
use crate::artifacts::dwg::schema::mutations::{DwgMutation, apply_dwg_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut DwgSnapshot, mutation: &DwgMutation) {
    apply_dwg_mutation(projection, mutation);
}

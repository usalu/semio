use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::DwgSnapshot;
use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::mutations::{DwgMutation, apply_dwg_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut DwgSnapshot, mutation: &DwgMutation) {
    apply_dwg_mutation(projection, mutation);
}

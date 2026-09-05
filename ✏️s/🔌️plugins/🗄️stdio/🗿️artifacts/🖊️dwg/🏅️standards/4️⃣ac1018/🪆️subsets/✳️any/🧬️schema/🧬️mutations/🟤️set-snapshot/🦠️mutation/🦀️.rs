use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::mutations::{apply_dwg_mutation, DwgMutation};
use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::DwgSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut DwgSnapshot, mutation: &DwgMutation) {
    apply_dwg_mutation(projection, mutation);
}

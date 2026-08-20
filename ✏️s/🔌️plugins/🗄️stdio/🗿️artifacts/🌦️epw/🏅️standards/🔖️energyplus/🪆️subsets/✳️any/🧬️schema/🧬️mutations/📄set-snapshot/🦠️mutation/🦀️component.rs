use crate::artifacts::epw::standards::energyplus::subsets::any::schema::mutations::{apply_epw_mutation, EpwMutation};
use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut EpwSnapshot, mutation: &EpwMutation) {
    let _ = apply_epw_mutation(projection, mutation);
}

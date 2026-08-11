use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
use crate::artifacts::epw::standards::energyplus::subsets::any::schema::mutations::{EpwMutation, apply_epw_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut EpwSnapshot, mutation: &EpwMutation) {
    let _ = apply_epw_mutation(projection, mutation);
}

use crate::artifacts::epw::standards::energyplus::subsets::any::schema::mutations::{apply_epw_mutation, EpwMutation};
use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut EpwSnapshot, mutation: &EpwMutation) {
    let _ = apply_epw_mutation(projection, mutation);
}

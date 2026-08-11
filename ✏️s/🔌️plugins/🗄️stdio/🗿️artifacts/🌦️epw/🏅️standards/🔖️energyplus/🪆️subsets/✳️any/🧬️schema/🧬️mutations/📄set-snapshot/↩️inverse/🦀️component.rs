use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
use crate::artifacts::epw::standards::energyplus::subsets::any::schema::mutations::EpwMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &EpwSnapshot, mutation: &EpwMutation) -> Vec<EpwMutation> {
    <EpwMutation as Mutation<EpwSnapshot>>::inverse(mutation, base)
}

use crate::artifacts::epw::standards::energyplus::subsets::any::schema::mutations::EpwMutation;
use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &EpwSnapshot, mutation: &EpwMutation) -> Vec<EpwMutation> {
    <EpwMutation as Mutation<EpwSnapshot>>::inverse(mutation, base).await
}

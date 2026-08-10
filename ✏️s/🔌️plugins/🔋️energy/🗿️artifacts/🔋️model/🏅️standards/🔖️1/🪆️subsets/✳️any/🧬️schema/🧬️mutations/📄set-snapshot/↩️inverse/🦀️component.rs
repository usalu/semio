use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::mutations::EnergyModelMutation;
use protocol::Mutation;

pub fn inverse(base: &EnergyModelSnapshot, mutation: &EnergyModelMutation) -> Vec<EnergyModelMutation> {
    <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::inverse(mutation, base)
}

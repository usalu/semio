use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::mutations::EnergyModelMutation;

pub fn apply(projection: &mut EnergyModelSnapshot, mutation: &EnergyModelMutation) {
    *projection = {
        let mut next = projection.clone();
        crate::artifacts::model::mutations::apply_energy_model_mutation(&mut next, mutation);
        next
    };
}

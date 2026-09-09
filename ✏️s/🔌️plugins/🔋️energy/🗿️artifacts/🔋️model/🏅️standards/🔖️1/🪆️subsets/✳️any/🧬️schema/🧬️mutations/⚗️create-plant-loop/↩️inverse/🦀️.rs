//! ↩️ Inverse for `CreatePlantLoop` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreatePlantLoop, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if (base.model.plant_loops.iter().any(|item| item.id == payload.id))
        || (payload.name.trim().is_empty())
        || (base.model.plant_loops.iter().any(|item| item.name == payload.name))
        || (!payload.supply_temperature_c.is_finite() || !(-100.0..=300.0).contains(&payload.supply_temperature_c))
        || (!payload.return_temperature_c.is_finite() || !(-100.0..=300.0).contains(&payload.return_temperature_c))
        || (!payload.design_flow_kg_s.is_finite() || payload.design_flow_kg_s <= 0.0)
        || (payload.equipment_ids.windows(2).any(|pair| pair[0].0 >= pair[1].0))
        || (payload.equipment_ids.iter().any(|entry| entry.0 == 0))
    {
        return Vec::new();
    }
    vec![vocabulary::delete_plant_loop(payload.id)]
}
//#endregion 🔖️Inverse

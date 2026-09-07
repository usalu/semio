//! ↩️ Inverse for `ChangeIdealLoadsSystemMaxCoolingCapacity` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeIdealLoadsSystemMaxCoolingCapacity, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let value = payload.new_capacity_present.then_some(payload.new_max_cooling_capacity_w);
    match base.model.ideal_loads.iter().find(|item| item.id == payload.id) {
        Some(item) if item.max_cooling_capacity_w != value && !((!payload.new_capacity_present && payload.new_max_cooling_capacity_w != 0.0) || (payload.new_capacity_present && (!payload.new_max_cooling_capacity_w.is_finite() || payload.new_max_cooling_capacity_w <= 0.0))) => vec![vocabulary::change_ideal_loads_system_max_cooling_capacity(payload.id, item.max_cooling_capacity_w.is_some(), item.max_cooling_capacity_w.unwrap_or(0.0))],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

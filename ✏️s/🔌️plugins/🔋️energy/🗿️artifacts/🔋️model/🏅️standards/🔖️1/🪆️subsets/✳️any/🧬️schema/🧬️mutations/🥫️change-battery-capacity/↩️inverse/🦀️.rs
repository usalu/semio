//! ↩️ Inverse for `ChangeBatteryCapacity` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeBatteryCapacity, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.battery_storage.iter().find(|item| item.id == payload.id) {
        Some(item) if item.capacity_kwh != payload.new_capacity_kwh && !(!payload.new_capacity_kwh.is_finite() || payload.new_capacity_kwh <= 0.0) => vec![vocabulary::change_battery_capacity(payload.id, item.capacity_kwh)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

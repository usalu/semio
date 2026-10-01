//! ↩️ Inverse for `ChangeBatteryMaxDischarge` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeBatteryMaxDischarge, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.battery_storage.iter().find(|item| item.id == payload.id) {
        Some(item) if item.max_discharge_w != payload.new_max_discharge_w && !(!payload.new_max_discharge_w.is_finite() || payload.new_max_discharge_w <= 0.0) => vec![vocabulary::change_battery_max_discharge(payload.id, item.max_discharge_w)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

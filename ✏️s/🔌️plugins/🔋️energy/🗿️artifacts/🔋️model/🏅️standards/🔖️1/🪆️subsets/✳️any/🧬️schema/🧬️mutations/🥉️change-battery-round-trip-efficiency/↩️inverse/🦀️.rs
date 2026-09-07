//! ↩️ Inverse for `ChangeBatteryRoundTripEfficiency` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeBatteryRoundTripEfficiency, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.battery_storage.iter().find(|item| item.id == payload.id) {
        Some(item) if item.round_trip_efficiency != payload.new_round_trip_efficiency && !(!(payload.new_round_trip_efficiency > 0.0 && payload.new_round_trip_efficiency <= 1.0)) => vec![vocabulary::change_battery_round_trip_efficiency(payload.id, item.round_trip_efficiency)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

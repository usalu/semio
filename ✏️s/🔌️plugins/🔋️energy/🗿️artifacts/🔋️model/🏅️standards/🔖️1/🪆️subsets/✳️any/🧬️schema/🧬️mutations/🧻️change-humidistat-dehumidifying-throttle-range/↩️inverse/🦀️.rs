//! ↩️ Inverse for `ChangeHumidistatDehumidifyingThrottleRange` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeHumidistatDehumidifyingThrottleRange, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.humidistats.iter().find(|item| item.id == payload.id) {
        Some(item) if item.dehumidifying_throttle_range != payload.new_dehumidifying_throttle_range && !(!payload.new_dehumidifying_throttle_range.is_finite() || payload.new_dehumidifying_throttle_range <= 0.0) => vec![vocabulary::change_humidistat_dehumidifying_throttle_range(payload.id, item.dehumidifying_throttle_range)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

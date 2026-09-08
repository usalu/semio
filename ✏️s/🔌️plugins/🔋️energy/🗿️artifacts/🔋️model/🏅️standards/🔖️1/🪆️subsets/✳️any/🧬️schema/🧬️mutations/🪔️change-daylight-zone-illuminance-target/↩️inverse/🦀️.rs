//! ↩️ Inverse for `ChangeDaylightZoneIlluminanceTarget` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeDaylightZoneIlluminanceTarget, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.daylight_zones.iter().find(|item| item.id == payload.id) {
        Some(item) if item.illuminance_target_lux != payload.new_illuminance_target_lux && !(!payload.new_illuminance_target_lux.is_finite() || payload.new_illuminance_target_lux <= 0.0) => vec![vocabulary::change_daylight_zone_illuminance_target(payload.id, item.illuminance_target_lux)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

//! ↩️ Inverse for `ChangeDaylightZoneIlluminanceTarget` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeDaylightZoneIlluminanceTarget, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.daylight_zones.iter().find(|item| item.id == payload.id) {
        Some(item) if item.illuminance_target_lux != payload.new_illuminance_target_lux && !(!payload.new_illuminance_target_lux.is_finite() || payload.new_illuminance_target_lux <= 0.0) => vec![vocabulary::change_daylight_zone_illuminance_target(payload.id, item.illuminance_target_lux)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

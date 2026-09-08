//! ↩️ Inverse for `ChangeDaylightZoneWindowTransmittance` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeDaylightZoneWindowTransmittance, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.daylight_zones.iter().find(|item| item.id == payload.id) {
        Some(item) if item.window_transmittance != payload.new_window_transmittance && !(!payload.new_window_transmittance.is_finite() || !(0.0..=1.0).contains(&payload.new_window_transmittance)) => vec![vocabulary::change_daylight_zone_window_transmittance(payload.id, item.window_transmittance)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

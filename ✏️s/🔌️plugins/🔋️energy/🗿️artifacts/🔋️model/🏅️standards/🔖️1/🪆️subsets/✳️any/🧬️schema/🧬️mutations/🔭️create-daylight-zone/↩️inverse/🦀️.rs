//! ↩️ Inverse for `CreateDaylightZone` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateDaylightZone, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if (base.model.daylight_zones.iter().any(|item| item.id == payload.id)) || (!base.model.zones.iter().any(|zone| zone.id == payload.zone_id)) || (!payload.illuminance_target_lux.is_finite() || payload.illuminance_target_lux <= 0.0) || (!payload.glare_limit.is_finite() || payload.glare_limit <= 0.0) || (!payload.window_transmittance.is_finite() || !(0.0..=1.0).contains(&payload.window_transmittance)) {
        return Vec::new();
    }
    vec![vocabulary::delete_daylight_zone(payload.id)]
}
//#endregion 🔖️Inverse

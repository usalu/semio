//! ↩️ Inverse for `DeleteDaylightZone` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteDaylightZone, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.daylight_zones.iter().find(|item| item.id == payload.id) {
        Some(item) => vec![vocabulary::create_daylight_zone(item.id, item.zone_id, item.illuminance_target_lux, item.glare_limit, item.window_transmittance)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

//! ↩️ Inverse for `ChangeDaylightZoneGlareLimit` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeDaylightZoneGlareLimit, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.daylight_zones.iter().find(|item| item.id == payload.id) {
        Some(item) if item.glare_limit != payload.new_glare_limit && !(!payload.new_glare_limit.is_finite() || payload.new_glare_limit <= 0.0) => vec![vocabulary::change_daylight_zone_glare_limit(payload.id, item.glare_limit)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

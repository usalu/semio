//! ↩️ Inverse for `ChangeDaylightZoneGlareLimit` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeDaylightZoneGlareLimit, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeDaylightZoneGlareLimit>,
{
    Ok((|| {
    match base.model.daylight_zones.iter().find(|item| item.id == payload.id) {
        Some(item) if item.glare_limit != payload.new_glare_limit && !(!payload.new_glare_limit.is_finite() || payload.new_glare_limit <= 0.0) => vec![vocabulary::change_daylight_zone_glare_limit(payload.id, item.glare_limit)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse

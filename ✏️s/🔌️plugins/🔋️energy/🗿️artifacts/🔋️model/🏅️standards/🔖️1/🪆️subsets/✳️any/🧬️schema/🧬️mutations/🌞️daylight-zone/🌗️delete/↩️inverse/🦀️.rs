//! ↩️ Inverse for `DeleteDaylightZone` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::DeleteDaylightZone, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::DeleteDaylightZone> + From<crate::CreateDaylightZone>,
{
    Ok((|| {
    match base.model.daylight_zones.iter().enumerate().find(|(_, item)| item.id == payload.id) {
        Some((index, item)) => vec![vocabulary::create_daylight_zone(item.id, item.zone_id, item.illuminance_target_lux, item.glare_limit, item.window_transmittance, Some(index as u32))],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse

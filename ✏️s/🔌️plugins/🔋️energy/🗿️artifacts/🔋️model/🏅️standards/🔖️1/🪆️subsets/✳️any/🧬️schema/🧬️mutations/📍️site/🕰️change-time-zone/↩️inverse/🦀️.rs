//! ↩️ Inverse for `ChangeSiteTimeZone` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeSiteTimeZone, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeSiteTimeZone>,
{
    Ok((|| {
    if !(-12.0..=14.0).contains(&payload.new_time_zone_hours) || base.model.site.time_zone_hours == payload.new_time_zone_hours {
        return Vec::new();
    }
    vec![vocabulary::change_site_time_zone(base.model.site.time_zone_hours)]

    })())
}
//#endregion 🔖️Inverse

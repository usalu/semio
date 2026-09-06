//! ↩️ Inverse for `UpdateSite` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::UpdateSite, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let site = crate::model::Site { latitude_deg: payload.latitude_deg, longitude_deg: payload.longitude_deg, elevation_m: payload.elevation_m, time_zone_hours: payload.time_zone_hours, north_axis_deg: payload.north_axis_deg };
    if base.model.site == site {
        return Vec::new();
    }
    let old = base.model.site;
    vec![vocabulary::update_site(old.latitude_deg, old.longitude_deg, old.elevation_m, old.time_zone_hours, old.north_axis_deg)]
}
//#endregion 🔖️Inverse

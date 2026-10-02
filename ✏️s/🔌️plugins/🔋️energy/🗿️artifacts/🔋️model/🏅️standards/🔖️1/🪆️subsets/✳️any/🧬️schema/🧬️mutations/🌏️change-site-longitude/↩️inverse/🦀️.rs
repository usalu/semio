//! ↩️ Inverse for `ChangeSiteLongitude` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeSiteLongitude, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if !(-180.0..=180.0).contains(&payload.new_longitude_deg) || base.model.site.longitude_deg == payload.new_longitude_deg {
        return Vec::new();
    }
    vec![vocabulary::change_site_longitude(base.model.site.longitude_deg)]
}
//#endregion 🔖️Inverse

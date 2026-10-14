//! ↩️ Inverse for `ChangeSiteElevation` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeSiteElevation, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if !(-300.0..8900.0).contains(&payload.new_elevation_m) || base.model.site.elevation_m == payload.new_elevation_m {
        return Vec::new();
    }
    vec![vocabulary::change_site_elevation(base.model.site.elevation_m)]

    })())
}
//#endregion 🔖️Inverse

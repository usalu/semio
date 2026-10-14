//! ↩️ Inverse for `ChangeSiteElevation` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeSiteElevation, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeSiteElevation>,
{
    Ok((|| {
    if !(-300.0..8900.0).contains(&payload.new_elevation_m) || base.model.site.elevation_m == payload.new_elevation_m {
        return Vec::new();
    }
    vec![vocabulary::change_site_elevation(base.model.site.elevation_m)]

    })())
}
//#endregion 🔖️Inverse

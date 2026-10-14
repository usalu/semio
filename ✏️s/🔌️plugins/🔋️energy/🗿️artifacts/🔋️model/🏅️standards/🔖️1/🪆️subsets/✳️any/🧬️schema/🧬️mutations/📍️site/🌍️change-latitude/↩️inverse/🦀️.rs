//! ↩️ Inverse for `ChangeSiteLatitude` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeSiteLatitude, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if !(-90.0..=90.0).contains(&payload.new_latitude_deg) || base.model.site.latitude_deg == payload.new_latitude_deg {
        return Vec::new();
    }
    vec![vocabulary::change_site_latitude(base.model.site.latitude_deg)]

    })())
}
//#endregion 🔖️Inverse

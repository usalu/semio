//! ↩️ Inverse for `ChangeSiteLatitude` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeSiteLatitude, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeSiteLatitude>,
{
    Ok((|| {
    if !(-90.0..=90.0).contains(&payload.new_latitude_deg) || base.model.site.latitude_deg == payload.new_latitude_deg {
        return Vec::new();
    }
    vec![vocabulary::change_site_latitude(base.model.site.latitude_deg)]

    })())
}
//#endregion 🔖️Inverse

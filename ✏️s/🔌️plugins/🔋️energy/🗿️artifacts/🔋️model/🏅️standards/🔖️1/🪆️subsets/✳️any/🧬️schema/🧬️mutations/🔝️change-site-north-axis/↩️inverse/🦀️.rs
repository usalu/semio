//! ↩️ Inverse for `ChangeSiteNorthAxis` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeSiteNorthAxis, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if !payload.new_north_axis_deg.is_finite() || base.model.site.north_axis_deg == payload.new_north_axis_deg {
        return Vec::new();
    }
    vec![vocabulary::change_site_north_axis(base.model.site.north_axis_deg)]
}
//#endregion 🔖️Inverse

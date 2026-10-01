//! ↩️ Inverse for `ChangeSurfaceWindExposed` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeSurfaceWindExposed, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if existing.wind_exposed == payload.new_wind_exposed {
        return Vec::new();
    }
    vec![vocabulary::change_surface_wind_exposed(payload.id, existing.wind_exposed)]
}
//#endregion 🔖️Inverse

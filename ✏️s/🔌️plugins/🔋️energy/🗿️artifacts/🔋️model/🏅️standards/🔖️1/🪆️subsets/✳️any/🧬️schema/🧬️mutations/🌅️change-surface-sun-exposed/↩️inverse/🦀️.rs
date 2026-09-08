//! ↩️ Inverse for `ChangeSurfaceSunExposed` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeSurfaceSunExposed, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if existing.sun_exposed == payload.new_sun_exposed {
        return Vec::new();
    }
    vec![vocabulary::change_surface_sun_exposed(payload.id, existing.sun_exposed)]
}
//#endregion 🔖️Inverse

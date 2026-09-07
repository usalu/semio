//! ↩️ Inverse for `ChangeSurfaceWindExposed` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

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

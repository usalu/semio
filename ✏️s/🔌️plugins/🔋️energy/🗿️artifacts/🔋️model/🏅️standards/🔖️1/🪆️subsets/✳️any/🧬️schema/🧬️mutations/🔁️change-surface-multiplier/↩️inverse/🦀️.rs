//! ↩️ Inverse for `ChangeSurfaceMultiplier` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeSurfaceMultiplier, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if payload.new_multiplier == 0 || existing.multiplier == payload.new_multiplier {
        return Vec::new();
    }
    vec![vocabulary::change_surface_multiplier(payload.id, existing.multiplier)]
}
//#endregion 🔖️Inverse

//! ↩️ Inverse for `ChangeFenestrationSurface` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeFenestrationSurface, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if !base.model.surfaces.iter().any(|item| item.id == payload.new_surface_id) || existing.surface_id == payload.new_surface_id {
        return Vec::new();
    }
    vec![vocabulary::change_fenestration_surface(payload.id, existing.surface_id)]
}
//#endregion 🔖️Inverse

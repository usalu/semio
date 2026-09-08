//! ↩️ Inverse for `ChangeSurfaceConstruction` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeSurfaceConstruction, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if !base.model.constructions.iter().any(|item| item.id == payload.new_construction_id) || existing.construction_id == payload.new_construction_id {
        return Vec::new();
    }
    vec![vocabulary::change_surface_construction(payload.id, existing.construction_id)]
}
//#endregion 🔖️Inverse

//! ↩️ Inverse for `ChangeSurfaceConstruction` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

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

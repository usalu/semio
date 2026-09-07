//! ↩️ Inverse for `DeleteConstruction` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteConstruction, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(index) = base.model.constructions.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if base.model.surfaces.iter().any(|surface| surface.construction_id == payload.id) {
        return Vec::new();
    }
    let existing = &base.model.constructions[index];
    vec![vocabulary::create_construction(index as u32, existing.id, existing.name.clone(), existing.layer_material_ids.clone())]
}
//#endregion 🔖️Inverse

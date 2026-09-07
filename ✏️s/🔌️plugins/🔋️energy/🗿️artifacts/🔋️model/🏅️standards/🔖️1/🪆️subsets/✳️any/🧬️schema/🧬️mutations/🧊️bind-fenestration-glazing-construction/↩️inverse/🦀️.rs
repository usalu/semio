//! ↩️ Inverse for `BindFenestrationGlazingConstruction` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::BindFenestrationGlazingConstruction, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if !base.model.constructions.iter().any(|item| item.id == payload.construction_id) {
        return Vec::new();
    }
    match existing.glazing_construction_id {
        Some(previous) if previous == payload.construction_id => Vec::new(),
        Some(previous) => vec![vocabulary::bind_fenestration_glazing_construction(payload.id, previous)],
        None => vec![vocabulary::clear_fenestration_glazing_construction(payload.id)],
    }
}
//#endregion 🔖️Inverse

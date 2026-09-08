//! ↩️ Inverse for `AddConstructionLayer` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::AddConstructionLayer, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.constructions.iter().find(|item| item.id == payload.id) {
        Some(construction) if base.model.materials.iter().any(|material| material.id == payload.material_id) && payload.index as usize <= construction.layer_material_ids.len() => vec![vocabulary::remove_construction_layer(payload.id, payload.index)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

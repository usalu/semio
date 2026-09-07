//! ↩️ Inverse for `RemoveConstructionLayer` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::RemoveConstructionLayer, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.constructions.iter().find(|item| item.id == payload.id) {
        Some(construction) => match construction.layer_material_ids.get(payload.index as usize) {
            Some(material_id) => vec![vocabulary::add_construction_layer(payload.id, payload.index, *material_id)],
            None => Vec::new(),
        },
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse

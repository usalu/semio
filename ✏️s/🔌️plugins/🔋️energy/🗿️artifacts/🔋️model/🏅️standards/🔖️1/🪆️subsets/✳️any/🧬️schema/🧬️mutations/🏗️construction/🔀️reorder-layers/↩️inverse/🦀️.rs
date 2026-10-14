//! ↩️ Inverse for `ReorderConstructionLayers` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ReorderConstructionLayers, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ReorderConstructionLayers>,
{
    Ok((|| {
    match base.model.constructions.iter().find(|item| item.id == payload.id) {
        Some(construction) => {
            let mut wanted: Vec<u32> = payload.new_layer_material_ids.iter().map(|id| id.0).collect();
            let mut held: Vec<u32> = construction.layer_material_ids.iter().map(|id| id.0).collect();
            wanted.sort_unstable();
            held.sort_unstable();
            if wanted != held || construction.layer_material_ids == payload.new_layer_material_ids {
                return Vec::new();
            }
            vec![vocabulary::reorder_construction_layers(payload.id, construction.layer_material_ids.clone())]
        }
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse

//! ↩️ Inverse for `AddConstructionLayer` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::AddConstructionLayer, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::AddConstructionLayer> + From<crate::RemoveConstructionLayer>,
{
    Ok((|| {
    match base.model.constructions.iter().find(|item| item.id == payload.id) {
        Some(construction) if base.model.materials.iter().any(|material| material.id == payload.material_id) && payload.index as usize <= construction.layer_material_ids.len() => vec![vocabulary::remove_construction_layer(payload.id, payload.index)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse

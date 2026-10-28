//! ↩️ Inverse for `ChangeMaterialSpecificHeat` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeMaterialSpecificHeat, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeMaterialSpecificHeat>,
{
    Ok((|| {
    match base.model.materials.iter().find(|item| item.id == payload.id) {
        Some(item) if item.specific_heat_j_kg_k != payload.new_specific_heat_j_kg_k && !(!payload.new_specific_heat_j_kg_k.is_finite() || payload.new_specific_heat_j_kg_k <= 0.0) => {
            vec![vocabulary::change_material_specific_heat(payload.id, item.specific_heat_j_kg_k)]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse

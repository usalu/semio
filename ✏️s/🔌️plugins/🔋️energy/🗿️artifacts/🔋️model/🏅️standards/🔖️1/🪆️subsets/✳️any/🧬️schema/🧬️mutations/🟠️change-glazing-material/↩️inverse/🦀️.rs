//! ↩️ Inverse for `ChangeGlazingMaterialConductivity` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeGlazingMaterialConductivity, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.glazing_materials.iter().find(|item| item.id == payload.id) {
        Some(item) if item.conductivity_w_m_k != payload.new_conductivity_w_m_k && payload.new_conductivity_w_m_k.is_finite() && payload.new_conductivity_w_m_k > 0.0 => vec![vocabulary::change_glazing_material_conductivity(payload.id, item.conductivity_w_m_k)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

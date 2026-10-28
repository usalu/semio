//! ↩️ Inverse for `ChangeGlazingMaterialConductivity` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeGlazingMaterialConductivity, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeGlazingMaterialConductivity>,
{
    Ok((|| {
    match base.model.glazing_materials.iter().find(|item| item.id == payload.id) {
        Some(item) if item.conductivity_w_m_k != payload.new_conductivity_w_m_k && payload.new_conductivity_w_m_k.is_finite() && payload.new_conductivity_w_m_k > 0.0 => vec![vocabulary::change_glazing_material_conductivity(payload.id, item.conductivity_w_m_k)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse

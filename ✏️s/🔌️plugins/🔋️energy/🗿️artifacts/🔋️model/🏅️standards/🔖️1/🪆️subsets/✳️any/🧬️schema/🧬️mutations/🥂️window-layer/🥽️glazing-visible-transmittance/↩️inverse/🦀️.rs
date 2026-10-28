//! ↩️ Inverse for `ChangeGlazingMaterialVisibleTransmittance` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeGlazingMaterialVisibleTransmittance, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeGlazingMaterialVisibleTransmittance>,
{
    Ok((|| {
    match base.model.glazing_materials.iter().find(|item| item.id == payload.id) {
        Some(item) if item.visible_transmittance != payload.new_visible_transmittance && payload.new_visible_transmittance.is_finite() && (0.0..=1.0).contains(&payload.new_visible_transmittance) => vec![vocabulary::change_glazing_material_visible_transmittance(payload.id, item.visible_transmittance)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse

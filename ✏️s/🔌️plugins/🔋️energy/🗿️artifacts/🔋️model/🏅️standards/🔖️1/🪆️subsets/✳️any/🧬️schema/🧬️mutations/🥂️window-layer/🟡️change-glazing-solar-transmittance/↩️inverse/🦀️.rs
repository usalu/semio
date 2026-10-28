//! ↩️ Inverse for `ChangeGlazingMaterialSolarTransmittance` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeGlazingMaterialSolarTransmittance, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeGlazingMaterialSolarTransmittance>,
{
    Ok((|| {
    match base.model.glazing_materials.iter().find(|item| item.id == payload.id) {
        Some(item) if item.solar_transmittance != payload.new_solar_transmittance && payload.new_solar_transmittance.is_finite() && (0.0..=1.0).contains(&payload.new_solar_transmittance) => vec![vocabulary::change_glazing_material_solar_transmittance(payload.id, item.solar_transmittance)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse

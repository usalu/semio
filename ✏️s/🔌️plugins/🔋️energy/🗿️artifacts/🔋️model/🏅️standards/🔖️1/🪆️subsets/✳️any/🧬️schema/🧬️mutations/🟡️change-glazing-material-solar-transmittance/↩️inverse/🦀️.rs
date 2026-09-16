//! ↩️ Inverse for `ChangeGlazingMaterialSolarTransmittance` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeGlazingMaterialSolarTransmittance, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.glazing_materials.iter().find(|item| item.id == payload.id) {
        Some(item) if item.solar_transmittance != payload.new_solar_transmittance && payload.new_solar_transmittance.is_finite() && (0.0..=1.0).contains(&payload.new_solar_transmittance) => vec![vocabulary::change_glazing_material_solar_transmittance(payload.id, item.solar_transmittance)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

//! ↩️ Inverse for `ChangeMechanicalVentilationFanDeltaPressure` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeMechanicalVentilationFanDeltaPressure, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.mechanical_ventilations.iter().find(|item| item.id == payload.id) {
        Some(item) if item.fan_delta_pressure_pa != payload.new_fan_delta_pressure_pa && !(!payload.new_fan_delta_pressure_pa.is_finite() || payload.new_fan_delta_pressure_pa < 0.0) => vec![vocabulary::change_mechanical_ventilation_fan_delta_pressure(payload.id, item.fan_delta_pressure_pa)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

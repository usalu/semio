//! ↩️ Inverse for `ChangeMechanicalVentilationFanTotalEfficiency` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeMechanicalVentilationFanTotalEfficiency, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.mechanical_ventilations.iter().find(|item| item.id == payload.id) {
        Some(item) if item.fan_total_efficiency != payload.new_fan_total_efficiency && !(!(payload.new_fan_total_efficiency > 0.0 && payload.new_fan_total_efficiency <= 1.0)) => vec![vocabulary::change_mechanical_ventilation_fan_total_efficiency(payload.id, item.fan_total_efficiency)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

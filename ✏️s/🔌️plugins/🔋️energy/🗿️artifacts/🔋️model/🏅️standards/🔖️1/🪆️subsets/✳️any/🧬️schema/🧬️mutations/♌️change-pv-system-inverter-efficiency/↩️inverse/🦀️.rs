//! ↩️ Inverse for `ChangePvSystemInverterEfficiency` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangePvSystemInverterEfficiency, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.pv_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.inverter_efficiency != payload.new_inverter_efficiency && !(!(payload.new_inverter_efficiency > 0.0 && payload.new_inverter_efficiency <= 1.0)) => vec![vocabulary::change_pv_system_inverter_efficiency(payload.id, item.inverter_efficiency)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

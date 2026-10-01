//! ↩️ Inverse for `ChangePvSystemModuleEfficiency` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangePvSystemModuleEfficiency, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.pv_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.module_efficiency != payload.new_module_efficiency && !(!(payload.new_module_efficiency > 0.0 && payload.new_module_efficiency <= 1.0)) => {
            vec![vocabulary::change_pv_system_module_efficiency(payload.id, item.module_efficiency)]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

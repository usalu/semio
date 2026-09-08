//! ↩️ Inverse for `ChangeShwSystemHeaterCapacity` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeShwSystemHeaterCapacity, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.shw_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.heater_capacity_w != payload.new_heater_capacity_w && !(!payload.new_heater_capacity_w.is_finite() || payload.new_heater_capacity_w <= 0.0) => vec![vocabulary::change_shw_system_heater_capacity(payload.id, item.heater_capacity_w)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

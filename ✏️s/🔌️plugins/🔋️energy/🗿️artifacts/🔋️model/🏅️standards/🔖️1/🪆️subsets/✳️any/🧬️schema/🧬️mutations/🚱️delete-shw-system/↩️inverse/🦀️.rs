//! ↩️ Inverse for `DeleteShwSystem` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteShwSystem, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(index) = base.model.shw_systems.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let existing = &base.model.shw_systems[index];
    vec![vocabulary::create_shw_system(index as u32, existing.id, existing.heater_capacity_w, existing.storage_volume_m3, existing.setpoint_c, existing.schedule_id)]
}
//#endregion 🔖️Inverse

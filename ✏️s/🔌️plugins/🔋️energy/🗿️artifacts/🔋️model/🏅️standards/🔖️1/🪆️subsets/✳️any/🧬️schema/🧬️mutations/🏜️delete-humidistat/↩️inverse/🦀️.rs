//! ↩️ Inverse for `DeleteHumidistat` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteHumidistat, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.humidistats.iter().find(|item| item.id == payload.id) {
        Some(item) => vec![vocabulary::create_humidistat(item.id, item.zone_id, item.humidifying_setpoint_schedule_id, item.dehumidifying_setpoint_schedule_id, item.humidifying_throttle_range, item.dehumidifying_throttle_range)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

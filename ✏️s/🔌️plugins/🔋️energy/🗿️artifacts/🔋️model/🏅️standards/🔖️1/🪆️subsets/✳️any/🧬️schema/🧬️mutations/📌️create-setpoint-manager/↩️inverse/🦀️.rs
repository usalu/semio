//! ↩️ Inverse for `CreateSetpointManager` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateSetpointManager, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if (base.model.setpoint_managers.iter().any(|item| item.id == payload.id)) || (payload.name.trim().is_empty()) || (base.model.setpoint_managers.iter().any(|item| item.name == payload.name)) || (!matches!(payload.kind.as_str(), "Scheduled" | "OutdoorAirReset" | "WarmestZone" | "ColdestZone")) || (payload.kind != "OutdoorAirReset" && !(payload.low_outdoor_c == 0.0 && payload.high_outdoor_c == 0.0 && payload.low_setpoint_c == 0.0 && payload.high_setpoint_c == 0.0)) || (payload.kind == "OutdoorAirReset" && payload.high_outdoor_c <= payload.low_outdoor_c) || (!payload.schedule_present && payload.schedule_id.0 != 0) || (payload.schedule_present && (!(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.schedule_id)))) {
        return Vec::new();
    }
    vec![vocabulary::delete_setpoint_manager(payload.id)]
}
//#endregion 🔖️Inverse

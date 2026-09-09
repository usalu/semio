//! ↩️ Inverse for `CreateHumidistat` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateHumidistat, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if (base.model.humidistats.iter().any(|item| item.id == payload.id))
        || (!base.model.zones.iter().any(|zone| zone.id == payload.zone_id))
        || (!(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.humidifying_setpoint_schedule_id)
            || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.humidifying_setpoint_schedule_id)
            || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.humidifying_setpoint_schedule_id)
            || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.humidifying_setpoint_schedule_id)
            || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.humidifying_setpoint_schedule_id)))
        || (!(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.dehumidifying_setpoint_schedule_id)
            || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.dehumidifying_setpoint_schedule_id)
            || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.dehumidifying_setpoint_schedule_id)
            || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.dehumidifying_setpoint_schedule_id)
            || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.dehumidifying_setpoint_schedule_id)))
        || (!payload.humidifying_throttle_range.is_finite() || payload.humidifying_throttle_range <= 0.0)
        || (!payload.dehumidifying_throttle_range.is_finite() || payload.dehumidifying_throttle_range <= 0.0)
    {
        return Vec::new();
    }
    vec![vocabulary::delete_humidistat(payload.id)]
}
//#endregion 🔖️Inverse

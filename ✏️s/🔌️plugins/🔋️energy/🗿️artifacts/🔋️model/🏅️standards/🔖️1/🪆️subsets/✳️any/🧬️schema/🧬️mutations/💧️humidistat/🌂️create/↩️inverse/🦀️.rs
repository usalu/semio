//! ↩️ Inverse for `CreateHumidistat` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::CreateHumidistat, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::CreateHumidistat> + From<crate::DeleteHumidistat>,
{
    Ok((|| {
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
    if payload.index.is_some_and(|index| index as usize > base.model.humidistats.len()) {
        return Vec::new();
    }
    vec![vocabulary::delete_humidistat(payload.id)]

    })())
}
//#endregion 🔖️Inverse

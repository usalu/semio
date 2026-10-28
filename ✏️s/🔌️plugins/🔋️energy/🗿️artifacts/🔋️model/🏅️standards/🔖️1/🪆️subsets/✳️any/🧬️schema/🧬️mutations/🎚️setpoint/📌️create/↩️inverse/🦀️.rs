//! ↩️ Inverse for `CreateSetpointManager` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::CreateSetpointManager, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::CreateSetpointManager> + From<crate::DeleteSetpointManager>,
{
    Ok((|| {
    if (base.model.setpoint_managers.iter().any(|item| item.id == payload.id))
        || (payload.name.trim().is_empty())
        || (base.model.setpoint_managers.iter().any(|item| item.name == payload.name))
        || crate::model::SetpointManagerKind::from_parts(payload.kind,[payload.low_outdoor_c,payload.high_outdoor_c,payload.low_setpoint_c,payload.high_setpoint_c]).is_none()
        || (!payload.schedule_present && payload.schedule_id.0 != 0)
        || (payload.schedule_present
            && (!(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.schedule_id)
                || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.schedule_id)
                || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.schedule_id)
                || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.schedule_id)
                || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.schedule_id))))
    {
        return Vec::new();
    }
    if payload.index.is_some_and(|index| index as usize > base.model.setpoint_managers.len()) {
        return Vec::new();
    }
    vec![vocabulary::delete_setpoint_manager(payload.id)]

    })())
}
//#endregion 🔖️Inverse

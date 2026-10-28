//! ↩️ Inverse for `CreatePeopleGain` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::CreatePeopleGain, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::CreatePeopleGain> + From<crate::DeletePeopleGain>,
{
    Ok((|| {
    if base.model.people.iter().any(|item| item.id == payload.id)
        || payload.index as usize > base.model.people.len()
        || !base.model.zones.iter().any(|zone| zone.id == payload.zone_id)
        || !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.schedule_id)
            || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.schedule_id)
            || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.schedule_id)
            || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.schedule_id)
            || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.schedule_id))
        || !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.activity_schedule_id)
            || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.activity_schedule_id)
            || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.activity_schedule_id)
            || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.activity_schedule_id)
            || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.activity_schedule_id))
    {
        return Vec::new();
    }
    vec![vocabulary::delete_people_gain(payload.id)]

    })())
}
//#endregion 🔖️Inverse

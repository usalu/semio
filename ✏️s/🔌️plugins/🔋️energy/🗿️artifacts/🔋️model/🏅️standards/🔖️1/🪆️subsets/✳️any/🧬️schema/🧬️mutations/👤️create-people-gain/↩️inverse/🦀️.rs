//! ↩️ Inverse for `CreatePeopleGain` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreatePeopleGain, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if base.model.people.iter().any(|item| item.id == payload.id) || payload.index as usize > base.model.people.len() || !base.model.zones.iter().any(|zone| zone.id == payload.zone_id) || !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.schedule_id)) || !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.activity_schedule_id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.activity_schedule_id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.activity_schedule_id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.activity_schedule_id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.activity_schedule_id)) {
        return Vec::new();
    }
    vec![vocabulary::delete_people_gain(payload.id)]
}
//#endregion 🔖️Inverse

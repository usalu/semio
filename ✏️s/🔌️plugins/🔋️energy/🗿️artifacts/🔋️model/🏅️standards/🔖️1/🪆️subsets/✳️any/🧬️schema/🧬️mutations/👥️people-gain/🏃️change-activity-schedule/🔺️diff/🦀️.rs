//! 🔺️ Sparse diff builder for `ChangePeopleGainActivitySchedule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, PeopleGainPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePeopleGainActivitySchedule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.people.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("People Gain {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.new_activity_schedule_id)
        || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.new_activity_schedule_id)
        || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.new_activity_schedule_id)
        || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.new_activity_schedule_id)
        || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.new_activity_schedule_id))
    {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Schedule {} does not exist.", payload.new_activity_schedule_id.0), [payload.new_activity_schedule_id.0.to_string()]);
    }
    if existing.activity_schedule_id == payload.new_activity_schedule_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("People Gain {} already carries this schedule reference: {}.", payload.id.0, payload.new_activity_schedule_id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { people: Rows::modifying(PeopleGainPatch { activity_schedule_id: Some(payload.new_activity_schedule_id), ..PeopleGainPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

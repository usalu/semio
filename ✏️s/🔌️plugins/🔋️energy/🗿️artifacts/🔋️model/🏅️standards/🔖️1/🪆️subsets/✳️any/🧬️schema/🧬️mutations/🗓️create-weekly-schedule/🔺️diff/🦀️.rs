//! 🔺️ Sparse diff builder for `CreateWeeklySchedule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, ScheduleSetPatch, WeeklySchedulePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateWeeklySchedule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.id)
    {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Schedule {} is already defined.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.schedules.weekly.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Index {} is past the end of the model's {} weekly schedules.", payload.index, base.model.schedules.weekly.len()), [payload.id.0.to_string()]);
    }
    if payload.daily_schedule_ids.len() != 7 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A weekly schedule names seven daily profiles, got {}.", payload.daily_schedule_ids.len()), [payload.id.0.to_string()]);
    }
    if let Some(missing) = payload.daily_schedule_ids.iter().find(|candidate| !base.model.schedules.daily.iter().any(|row| row.id == **candidate)) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Daily schedule {} does not exist.", missing.0), [missing.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { schedules: ScheduleSetPatch { weekly: Rows::inserting(payload.index as usize, crate::schedule::WeeklySchedule {
            id: payload.id,
            daily_schedule_ids: {
                let mut ids = [crate::model::ScheduleId(0); 7];
                ids.copy_from_slice(&payload.daily_schedule_ids);
                ids
            },
        }), ..Default::default() }, ..Default::default() }))
}
//#endregion 🔖️Diff

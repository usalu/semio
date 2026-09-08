//! 🔺️ Sparse diff builder for `ChangeWeeklyScheduleDay` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeWeeklyScheduleDay, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.schedules.weekly.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Weekly schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.day_index > 6 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A week has seven days indexed 0 to 6, got {}.", payload.day_index), [payload.id.0.to_string()]);
    }
    if !base.model.schedules.daily.iter().any(|row| row.id == payload.new_daily_schedule_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Daily schedule {} does not exist.", payload.new_daily_schedule_id.0), [payload.new_daily_schedule_id.0.to_string()]);
    }
    if existing.daily_schedule_ids[payload.day_index as usize] == payload.new_daily_schedule_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Weekly schedule {} day {} already names that daily profile.", payload.id.0, payload.day_index));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.weekly.iter_mut().find(|item| item.id == payload.id) {
        item.daily_schedule_ids[payload.day_index as usize] = payload.new_daily_schedule_id;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

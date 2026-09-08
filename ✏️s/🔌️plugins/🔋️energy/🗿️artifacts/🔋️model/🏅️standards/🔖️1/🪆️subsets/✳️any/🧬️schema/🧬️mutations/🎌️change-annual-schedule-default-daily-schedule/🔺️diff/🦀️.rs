//! 🔺️ Sparse diff builder for `ChangeAnnualScheduleDefaultDailySchedule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeAnnualScheduleDefaultDailySchedule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.schedules.daily.iter().any(|row| row.id == payload.new_default_daily_schedule_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Daily schedule {} does not exist.", payload.new_default_daily_schedule_id.0), [payload.new_default_daily_schedule_id.0.to_string()]);
    }
    if existing.default_daily_schedule_id == payload.new_default_daily_schedule_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Annual schedule {} already carries this default_daily_schedule_id: {:?}.", payload.id.0, payload.new_default_daily_schedule_id));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.annual.iter_mut().find(|item| item.id == payload.id) {
        item.default_daily_schedule_id = payload.new_default_daily_schedule_id;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

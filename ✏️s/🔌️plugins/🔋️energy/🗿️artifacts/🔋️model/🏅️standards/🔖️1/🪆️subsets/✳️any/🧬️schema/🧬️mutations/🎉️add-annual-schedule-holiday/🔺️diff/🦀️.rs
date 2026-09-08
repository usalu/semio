//! 🔺️ Sparse diff builder for `AddAnnualScheduleHoliday` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::AddAnnualScheduleHoliday, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(1..=12).contains(&payload.month) || !(1..=31).contains(&payload.day) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("{}-{}-{} is not a calendar date.", payload.year, payload.month, payload.day), [payload.id.0.to_string()]);
    }
    if payload.index as usize > existing.holiday_dates.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of annual schedule {}'s {} holidays.", payload.index, payload.id.0, existing.holiday_dates.len()), [payload.id.0.to_string()]);
    }
    if existing.holiday_dates.contains(&(payload.year, payload.month, payload.day)) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Annual schedule {} already holds {}-{}-{} as a holiday.", payload.id.0, payload.year, payload.month, payload.day));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.annual.iter_mut().find(|item| item.id == payload.id) {
        item.holiday_dates.insert(payload.index as usize, (payload.year, payload.month, payload.day));
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

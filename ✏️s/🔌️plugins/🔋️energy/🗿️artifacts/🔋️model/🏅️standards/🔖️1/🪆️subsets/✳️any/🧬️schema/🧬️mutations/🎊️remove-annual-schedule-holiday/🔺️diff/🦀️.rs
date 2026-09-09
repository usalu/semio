//! 🔺️ Sparse diff builder for `RemoveAnnualScheduleHoliday` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveAnnualScheduleHoliday, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !existing.holiday_dates.contains(&(payload.year, payload.month, payload.day)) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} does not hold {}-{}-{} as a holiday.", payload.id.0, payload.year, payload.month, payload.day), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.annual.iter_mut().find(|item| item.id == payload.id) {
        item.holiday_dates.retain(|holiday| *holiday != (payload.year, payload.month, payload.day));
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

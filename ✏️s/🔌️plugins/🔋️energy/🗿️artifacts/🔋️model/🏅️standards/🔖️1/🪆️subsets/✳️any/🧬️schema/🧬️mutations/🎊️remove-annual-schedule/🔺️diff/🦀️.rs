//! 🔺️ Sparse diff builder for `RemoveAnnualScheduleHoliday` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, AnnualSchedulePatch, ListEdit, ModelPatch, Rows, ScheduleSetPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveAnnualScheduleHoliday, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !existing.holiday_dates.contains(&(payload.year, payload.month, payload.day)) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} does not hold {}-{}-{} as a holiday.", payload.id.0, payload.year, payload.month, payload.day), [payload.id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { schedules: ScheduleSetPatch { annual: Rows::modifying(AnnualSchedulePatch { holiday_dates: ListEdit::removing_where(&existing.holiday_dates, |holiday| *holiday == (payload.year, payload.month, payload.day)), ..AnnualSchedulePatch::of(payload.id) }), ..Default::default() }, ..Default::default() }))
}
//#endregion 🔖️Diff

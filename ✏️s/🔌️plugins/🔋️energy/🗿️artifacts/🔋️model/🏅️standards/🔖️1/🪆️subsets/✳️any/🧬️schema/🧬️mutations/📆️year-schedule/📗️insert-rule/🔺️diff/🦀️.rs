//! 🔺️ Sparse diff builder for `InsertAnnualScheduleRule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, AnnualSchedulePatch, ListEdit, ModelPatch, Rows, ScheduleSetPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::InsertAnnualScheduleRule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.index as usize > existing.rules.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Index {} is past the end of annual schedule {}'s {} rules.", payload.index, payload.id.0, existing.rules.len()), [payload.id.0.to_string()]);
    }
    if !(1..=12).contains(&payload.start_month) || !(1..=12).contains(&payload.end_month) || !(1..=31).contains(&payload.start_day) || !(1..=31).contains(&payload.end_day) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Rule {}-{} .. {}-{} is not a calendar interval.", payload.start_month, payload.start_day, payload.end_month, payload.end_day), [payload.id.0.to_string()]);
    }
    if !base.model.schedules.daily.iter().any(|row| row.id == payload.daily_schedule_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Daily schedule {} does not exist.", payload.daily_schedule_id.0), [payload.daily_schedule_id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { schedules: ScheduleSetPatch { annual: Rows::modifying(AnnualSchedulePatch { rules: ListEdit::inserting(payload.index as usize, crate::schedule::CompactScheduleRule { start_month: payload.start_month, start_day: payload.start_day, end_month: payload.end_month, end_day: payload.end_day, daily_schedule_id: payload.daily_schedule_id }), ..AnnualSchedulePatch::of(payload.id) }), ..Default::default() }, ..Default::default() }))
}
//#endregion 🔖️Diff

//! 🔺️ Sparse diff builder for `ReplaceDailyScheduleHourlyValues` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, DailySchedulePatch, ModelPatch, Rows, ScheduleSetPatch, Slots};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceDailyScheduleHourlyValues, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.schedules.daily.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Daily schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_hourly_values.len() != 24 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A daily schedule carries twenty-four hourly values, got {}.", payload.new_hourly_values.len()), [payload.id.0.to_string()]);
    }
    if payload.new_hourly_values.iter().any(|value| !value.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Every hourly schedule value must be finite.", [payload.id.0.to_string()]);
    }
    if existing.hourly_values.as_slice() == payload.new_hourly_values.as_slice() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Daily schedule {} already carries this hourly_values: {:?}.", payload.id.0, payload.new_hourly_values));
    }
    let mut values = [0.0f64; 24];
    values.copy_from_slice(&payload.new_hourly_values);
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { schedules: ScheduleSetPatch { daily: Rows::modifying(DailySchedulePatch { hourly_values: Slots::replacing(&existing.hourly_values, &values), ..DailySchedulePatch::of(payload.id) }), ..Default::default() }, ..Default::default() }))
}
//#endregion 🔖️Diff

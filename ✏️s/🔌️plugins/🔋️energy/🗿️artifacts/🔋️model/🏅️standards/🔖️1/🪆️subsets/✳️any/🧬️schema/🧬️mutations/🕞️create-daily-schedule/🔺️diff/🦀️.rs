//! 🔺️ Sparse diff builder for `CreateDailySchedule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateDailySchedule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Schedule {} is already defined.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.schedules.daily.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of the model's {} daily schedules.", payload.index, base.model.schedules.daily.len()), [payload.id.0.to_string()]);
    }
    if payload.hourly_values.len() != 24 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A daily schedule carries twenty-four hourly values, got {}.", payload.hourly_values.len()), [payload.id.0.to_string()]);
    }
    if payload.hourly_values.iter().any(|value| !value.is_finite()) {
        return protocol::MutationOutcome::error("mutation.invariant", "Every hourly schedule value must be finite.", [payload.id.0.to_string()]);
    }
    if payload.limits_min.is_some() != payload.limits_max.is_some() {
        return protocol::MutationOutcome::error("mutation.invariant", "Schedule limits are a lower and an upper bound together, or neither.", [payload.id.0.to_string()]);
    }
    if let (Some(min), Some(max)) = (payload.limits_min, payload.limits_max) {
        if !min.is_finite() || !max.is_finite() || min > max {
            return protocol::MutationOutcome::error("mutation.invariant", format!("Schedule limits {} .. {} are not an interval.", min, max), [payload.id.0.to_string()]);
        }
    }
    let mut model = base.model.clone();
    model.schedules.daily.insert(payload.index as usize, crate::schedule::DailySchedule { id: payload.id, hourly_values: { let mut values = [0.0f64; 24]; values.copy_from_slice(&payload.hourly_values); values }, interpolation: payload.interpolation, limits: match (payload.limits_min, payload.limits_max) { (Some(min), Some(max)) => Some(crate::schedule::ScheduleLimits { min, max }), _ => None } });
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

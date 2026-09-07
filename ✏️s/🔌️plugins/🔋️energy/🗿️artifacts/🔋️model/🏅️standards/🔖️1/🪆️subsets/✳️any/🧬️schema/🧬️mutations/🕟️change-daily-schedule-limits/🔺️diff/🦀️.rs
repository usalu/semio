//! 🔺️ Sparse diff builder for `ChangeDailyScheduleLimits` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeDailyScheduleLimits, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.schedules.daily.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Daily schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_limits_min.is_some() != payload.new_limits_max.is_some() {
        return protocol::MutationOutcome::error("mutation.invariant", "Schedule limits are a lower and an upper bound together, or neither.", [payload.id.0.to_string()]);
    }
    if let (Some(min), Some(max)) = (payload.new_limits_min, payload.new_limits_max) {
        if !min.is_finite() || !max.is_finite() || min > max {
            return protocol::MutationOutcome::error("mutation.invariant", format!("Schedule limits {} .. {} are not an interval.", min, max), [payload.id.0.to_string()]);
        }
    }
    let limits = match (payload.new_limits_min, payload.new_limits_max) {
        (Some(min), Some(max)) => Some(crate::schedule::ScheduleLimits { min, max }),
        _ => None,
    };
    if existing.limits == limits {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Daily schedule {} already carries these limits.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.daily.iter_mut().find(|item| item.id == payload.id) {
        item.limits = limits;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

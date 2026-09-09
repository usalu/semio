//! 🔺️ Sparse diff builder for `ChangeHumidistatHumidifyingThrottleRange` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeHumidistatHumidifyingThrottleRange, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.humidistats.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Humidistat {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_humidifying_throttle_range.is_finite() || payload.new_humidifying_throttle_range <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A humidifying throttle range must be a positive finite number, got {}.", payload.new_humidifying_throttle_range), [payload.id.0.to_string()]);
    }
    if existing.humidifying_throttle_range == payload.new_humidifying_throttle_range {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Humidistat {} already has that humidifying throttle range.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.humidistats.iter_mut().find(|item| item.id == payload.id) {
        item.humidifying_throttle_range = payload.new_humidifying_throttle_range;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

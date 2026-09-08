//! 🔺️ Sparse diff builder for `ChangeHumidistatDehumidifyingThrottleRange` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeHumidistatDehumidifyingThrottleRange, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.humidistats.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Humidistat {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_dehumidifying_throttle_range.is_finite() || payload.new_dehumidifying_throttle_range <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A dehumidifying throttle range must be a positive finite number, got {}.", payload.new_dehumidifying_throttle_range), [payload.id.0.to_string()]);
    }
    if existing.dehumidifying_throttle_range == payload.new_dehumidifying_throttle_range {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Humidistat {} already has that dehumidifying throttle range.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.humidistats.iter_mut().find(|item| item.id == payload.id) {
        item.dehumidifying_throttle_range = payload.new_dehumidifying_throttle_range;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

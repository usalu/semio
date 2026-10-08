//! 🔺️ Sparse diff builder for `ChangeHumidistatDehumidifyingThrottleRange` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, HumidistatPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeHumidistatDehumidifyingThrottleRange, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.humidistats.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Humidistat {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_dehumidifying_throttle_range.is_finite() || payload.new_dehumidifying_throttle_range <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A dehumidifying throttle range must be a positive finite number, got {}.", payload.new_dehumidifying_throttle_range), [payload.id.0.to_string()]);
    }
    if existing.dehumidifying_throttle_range == payload.new_dehumidifying_throttle_range {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Humidistat {} already has that dehumidifying throttle range.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { humidistats: Rows::modifying(HumidistatPatch { dehumidifying_throttle_range: Some(payload.new_dehumidifying_throttle_range), ..HumidistatPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

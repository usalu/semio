//! 🔺️ Sparse diff builder for `ChangeThermostatHeatingThrottleRange` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, ThermostatPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeThermostatHeatingThrottleRange, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.thermostats.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Thermostat {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_heating_throttle_range_k.is_finite() || payload.new_heating_throttle_range_k <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A heating throttle range must be a positive finite number, got {}.", payload.new_heating_throttle_range_k), [payload.id.0.to_string()]);
    }
    if existing.heating_throttle_range_k == payload.new_heating_throttle_range_k {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Thermostat {} already has that heating throttle range.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { thermostats: Rows::modifying(ThermostatPatch { heating_throttle_range_k: Some(payload.new_heating_throttle_range_k), ..ThermostatPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

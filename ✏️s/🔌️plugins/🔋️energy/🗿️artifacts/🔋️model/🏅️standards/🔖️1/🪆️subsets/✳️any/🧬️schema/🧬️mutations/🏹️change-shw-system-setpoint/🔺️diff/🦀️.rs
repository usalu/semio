//! 🔺️ Sparse diff builder for `ChangeShwSystemSetpoint` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, ShwSystemConfigPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeShwSystemSetpoint, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.shw_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Service hot water system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_setpoint_c.is_finite() || !(0.0..=100.0).contains(&payload.new_setpoint_c) {
        return protocol::MutationOutcome::fatal(
            "mutation.invariant",
            format!("Service hot water system {}: tank setpoint (°C) must be a liquid-water temperature in [0, 100] °C, got {}.", payload.id.0, payload.new_setpoint_c),
            [payload.id.0.to_string()],
        );
    }
    if existing.setpoint_c == payload.new_setpoint_c {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Service hot water system {} already carries this setpoint_c: {}.", payload.id.0, payload.new_setpoint_c));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { shw_systems: Rows::modifying(ShwSystemConfigPatch { setpoint_c: Some(payload.new_setpoint_c), ..ShwSystemConfigPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

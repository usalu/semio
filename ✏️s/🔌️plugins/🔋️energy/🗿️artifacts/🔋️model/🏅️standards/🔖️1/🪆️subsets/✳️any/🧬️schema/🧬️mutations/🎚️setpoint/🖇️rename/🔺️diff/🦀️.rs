//! 🔺️ Sparse diff builder for `RenameSetpointManager` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, SetpointManagerPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RenameSetpointManager, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.setpoint_managers.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Setpoint manager {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_name.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A setpoint manager name must not be blank.".to_string(), [payload.id.0.to_string()]);
    }
    if base.model.setpoint_managers.iter().any(|item| item.id != payload.id && item.name == payload.new_name) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Another setpoint manager is already named {:?}.", payload.new_name), [payload.id.0.to_string()]);
    }
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Setpoint manager {} already has that name.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { setpoint_managers: Rows::modifying(SetpointManagerPatch { name: Some(payload.new_name.clone()), ..SetpointManagerPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

//! 🔺️ Sparse diff builder for `RenameAirLoop` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelAirLoopPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RenameAirLoop, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.air_loops.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Air loop {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_name.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A air loop name must not be blank.".to_string(), [payload.id.0.to_string()]);
    }
    if base.model.air_loops.iter().any(|item| item.id != payload.id && item.name == payload.new_name) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Another air loop is already named {:?}.", payload.new_name), [payload.id.0.to_string()]);
    }
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Air loop {} already has that name.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { air_loops: Rows::modifying(ModelAirLoopPatch { name: Some(payload.new_name.clone()), ..ModelAirLoopPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

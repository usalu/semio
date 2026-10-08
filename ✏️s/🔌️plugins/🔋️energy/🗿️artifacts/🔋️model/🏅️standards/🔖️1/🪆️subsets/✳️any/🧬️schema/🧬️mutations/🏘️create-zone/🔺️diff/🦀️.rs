//! 🔺️ Sparse diff builder for `CreateZone` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, ZonePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateZone, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.zones.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Zone {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.name.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A zone name must not be blank.", [payload.id.0.to_string()]);
    }
    if base.model.zones.iter().any(|item| item.name == payload.name) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Another zone is already named \"{}\".", payload.name), [payload.id.0.to_string()]);
    }
    if !payload.volume_m3.is_finite() || payload.volume_m3 <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Zone {} needs a positive finite volume, got {}.", payload.id.0, payload.volume_m3), [payload.id.0.to_string()]);
    }
    if payload.multiplier == 0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Zone {} needs at least one instance.", payload.id.0), [payload.id.0.to_string()]);
    }
    let position = payload.index.map_or_else(|| base.model.zones.iter().position(|item| item.id > payload.id).unwrap_or(base.model.zones.len()), |index| index as usize);
    if position > base.model.zones.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Index {} is past the end of the model's {} zones.", position, base.model.zones.len()), [payload.id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { zones: Rows::inserting(position, crate::model::Zone { id: payload.id, name: payload.name.clone(), volume_m3: payload.volume_m3, multiplier: payload.multiplier, conditioned: payload.conditioned, part_of_total_floor_area: payload.part_of_total_floor_area }), ..Default::default() }))
}
//#endregion 🔖️Diff

//! 🔺️ Sparse diff builder for `RenameThermalEnclosure` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, ThermalEnclosurePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RenameThermalEnclosure, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.thermal_enclosures.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Thermal enclosure {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_name.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A thermal enclosure name must not be blank.", [payload.id.0.to_string()]);
    }
    if base.model.thermal_enclosures.iter().any(|other| other.id != payload.id && other.name == payload.new_name) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Another thermal enclosure is already named \"{}\".", payload.new_name), [payload.id.0.to_string()]);
    }
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Thermal enclosure {} already carries this name: {}.", payload.id.0, payload.new_name));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { thermal_enclosures: Rows::modifying(ThermalEnclosurePatch { name: Some(payload.new_name.clone()), ..ThermalEnclosurePatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

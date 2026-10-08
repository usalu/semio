//! 🔺️ Sparse diff builder for `RenameMaterial` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, MaterialPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RenameMaterial, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.materials.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_name.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A material name must not be blank.", [payload.id.0.to_string()]);
    }
    if base.model.materials.iter().any(|other| other.id != payload.id && other.name == payload.new_name) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Another material is already named \"{}\".", payload.new_name), [payload.id.0.to_string()]);
    }
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Material {} already carries this name: {}.", payload.id.0, payload.new_name));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { materials: Rows::modifying(MaterialPatch { name: Some(payload.new_name.clone()), ..MaterialPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

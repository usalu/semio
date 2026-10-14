//! 🔺️ Sparse diff builder for `RenameGlazingMaterial` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, GlazingMaterialPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RenameGlazingMaterial, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.glazing_materials.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Glazing material {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_name.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A name must not be blank.", [payload.id.0.to_string()]);
    }
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Glazing material {} is already called {:?}.", payload.id.0, payload.new_name));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { glazing_materials: Rows::modifying(GlazingMaterialPatch { name: Some(payload.new_name.clone()), ..GlazingMaterialPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

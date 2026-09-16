//! 🔺️ Sparse diff builder for `RenameGlazingMaterial` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RenameGlazingMaterial, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.glazing_materials.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Glazing material {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_name.trim().is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "A name must not be blank.", [payload.id.0.to_string()]);
    }
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Glazing material {} is already called {:?}.", payload.id.0, payload.new_name));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.glazing_materials.iter_mut().find(|item| item.id == payload.id) {
        item.name = payload.new_name.clone();
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

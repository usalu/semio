//! 🔺️ Sparse diff builder for `RenameSurface` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RenameSurface, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Surface {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_name.trim().is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "A surface name must not be blank.", [payload.id.0.to_string()]);
    }
    if base.model.surfaces.iter().any(|item| item.id != payload.id && item.name == payload.new_name) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Another surface is already named \"{}\".", payload.new_name), [payload.id.0.to_string()]);
    }
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Surface {} already has this name.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.surfaces.iter_mut().find(|item| item.id == payload.id) {
        item.name = payload.new_name.clone();
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

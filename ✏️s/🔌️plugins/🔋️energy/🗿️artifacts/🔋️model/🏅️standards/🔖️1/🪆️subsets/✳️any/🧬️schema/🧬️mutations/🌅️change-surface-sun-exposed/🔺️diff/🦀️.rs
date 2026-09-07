//! 🔺️ Sparse diff builder for `ChangeSurfaceSunExposed` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSurfaceSunExposed, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Surface {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if existing.sun_exposed == payload.new_sun_exposed {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Surface {} already has this sun exposure.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.surfaces.iter_mut().find(|item| item.id == payload.id) {
        item.sun_exposed = payload.new_sun_exposed;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

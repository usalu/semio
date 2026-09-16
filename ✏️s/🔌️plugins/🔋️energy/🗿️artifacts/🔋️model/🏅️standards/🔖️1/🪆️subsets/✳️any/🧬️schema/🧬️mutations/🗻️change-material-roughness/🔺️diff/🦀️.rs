//! 🔺️ Sparse diff builder for `ChangeMaterialRoughness` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeMaterialRoughness, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.materials.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if existing.roughness == payload.new_roughness {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Material {} already carries this roughness: {:?}.", payload.id.0, payload.new_roughness));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.materials.iter_mut().find(|item| item.id == payload.id) {
        item.roughness = payload.new_roughness;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

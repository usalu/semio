//! 🔺️ Sparse diff builder for `ChangeMaterialThickness` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeMaterialThickness, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.materials.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_thickness_m.is_finite() || payload.new_thickness_m <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Material {}: thickness (m) must be a positive finite value, got {}.", payload.id.0, payload.new_thickness_m), [payload.id.0.to_string()]);
    }
    if existing.thickness_m == payload.new_thickness_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Material {} already carries this thickness (m): {}.", payload.id.0, payload.new_thickness_m));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.materials.iter_mut().find(|item| item.id == payload.id) {
        item.thickness_m = payload.new_thickness_m;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

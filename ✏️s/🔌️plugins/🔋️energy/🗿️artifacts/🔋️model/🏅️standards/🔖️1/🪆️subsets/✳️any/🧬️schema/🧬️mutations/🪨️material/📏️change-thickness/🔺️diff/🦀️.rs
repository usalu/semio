//! 🔺️ Sparse diff builder for `ChangeMaterialThickness` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, MaterialPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeMaterialThickness, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.materials.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_thickness_m.is_finite() || payload.new_thickness_m <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Material {}: thickness (m) must be a positive finite value, got {}.", payload.id.0, payload.new_thickness_m), [payload.id.0.to_string()]);
    }
    if existing.thickness_m == payload.new_thickness_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Material {} already carries this thickness (m): {}.", payload.id.0, payload.new_thickness_m));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { materials: Rows::modifying(MaterialPatch { thickness_m: Some(payload.new_thickness_m), ..MaterialPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

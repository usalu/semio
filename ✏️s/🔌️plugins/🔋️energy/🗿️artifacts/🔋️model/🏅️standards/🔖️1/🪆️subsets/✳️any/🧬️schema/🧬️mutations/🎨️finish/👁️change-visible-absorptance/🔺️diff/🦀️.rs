//! 🔺️ Sparse diff builder for `ChangeMaterialVisibleAbsorptance` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, MaterialPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeMaterialVisibleAbsorptance, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.materials.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(0.0..=1.0).contains(&payload.new_visible_absorptance) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Material {}: visible absorptance must be a fraction in [0, 1], got {}.", payload.id.0, payload.new_visible_absorptance), [payload.id.0.to_string()]);
    }
    if existing.visible_absorptance == payload.new_visible_absorptance {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Material {} already carries this visible absorptance: {}.", payload.id.0, payload.new_visible_absorptance));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { materials: Rows::modifying(MaterialPatch { visible_absorptance: Some(payload.new_visible_absorptance), ..MaterialPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

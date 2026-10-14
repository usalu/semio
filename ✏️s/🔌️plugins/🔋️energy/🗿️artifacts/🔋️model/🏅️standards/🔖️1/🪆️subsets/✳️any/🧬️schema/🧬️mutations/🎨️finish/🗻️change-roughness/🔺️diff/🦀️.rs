//! 🔺️ Sparse diff builder for `ChangeMaterialRoughness` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, MaterialPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeMaterialRoughness, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.materials.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if existing.roughness == payload.new_roughness {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Material {} already carries this roughness: {:?}.", payload.id.0, payload.new_roughness));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { materials: Rows::modifying(MaterialPatch { roughness: Some(payload.new_roughness), ..MaterialPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

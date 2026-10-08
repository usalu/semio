//! 🔺️ Sparse diff builder for `DeleteMaterial` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, MaterialPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteMaterial, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.materials.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    if base.model.constructions.iter().any(|construction| construction.layer_material_ids.contains(&payload.id)) {
        return protocol::MutationOutcome::error("mutation.target-referenced", format!("Material {} is still a layer of a construction.", payload.id.0), [payload.id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { materials: Rows::removing(&base.model.materials, &payload.id), ..Default::default() }))
}
//#endregion 🔖️Diff

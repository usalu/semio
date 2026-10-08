//! 🔺️ Sparse diff builder for `RenameGasMaterial` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, GasMaterialPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RenameGasMaterial, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.gas_materials.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Gas material {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_name.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A name must not be blank.", [payload.id.0.to_string()]);
    }
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Gas material {} is already called {:?}.", payload.id.0, payload.new_name));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { gas_materials: Rows::modifying(GasMaterialPatch { name: Some(payload.new_name.clone()), ..GasMaterialPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

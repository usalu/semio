//! 🔺️ Sparse diff builder for `ChangeGasMaterialGas` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, GasMaterialPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeGasMaterialGas, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.gas_materials.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Gas material {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if existing.gas == payload.new_gas {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Gas material {} is already filled with {:?}.", payload.id.0, payload.new_gas));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { gas_materials: Rows::modifying(GasMaterialPatch { gas: Some(payload.new_gas), ..GasMaterialPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

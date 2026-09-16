//! 🔺️ Sparse diff builder for `ChangeGasMaterialGas` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeGasMaterialGas, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.gas_materials.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Gas material {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if existing.gas == payload.new_gas {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Gas material {} is already filled with {:?}.", payload.id.0, payload.new_gas));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.gas_materials.iter_mut().find(|item| item.id == payload.id) {
        item.gas = payload.new_gas;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

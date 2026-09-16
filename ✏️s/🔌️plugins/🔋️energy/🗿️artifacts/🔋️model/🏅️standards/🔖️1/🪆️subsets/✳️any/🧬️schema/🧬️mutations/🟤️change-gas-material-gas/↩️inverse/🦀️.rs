//! ↩️ Inverse for `ChangeGasMaterialGas` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeGasMaterialGas, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.gas_materials.iter().find(|item| item.id == payload.id) {
        Some(item) if item.gas != payload.new_gas => vec![vocabulary::change_gas_material_gas(payload.id, item.gas)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

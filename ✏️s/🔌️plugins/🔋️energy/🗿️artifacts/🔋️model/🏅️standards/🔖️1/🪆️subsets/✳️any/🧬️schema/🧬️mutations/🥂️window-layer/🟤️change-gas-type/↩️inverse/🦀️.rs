//! ↩️ Inverse for `ChangeGasMaterialGas` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeGasMaterialGas, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeGasMaterialGas>,
{
    Ok((|| {
    match base.model.gas_materials.iter().find(|item| item.id == payload.id) {
        Some(item) if item.gas != payload.new_gas => vec![vocabulary::change_gas_material_gas(payload.id, item.gas)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse

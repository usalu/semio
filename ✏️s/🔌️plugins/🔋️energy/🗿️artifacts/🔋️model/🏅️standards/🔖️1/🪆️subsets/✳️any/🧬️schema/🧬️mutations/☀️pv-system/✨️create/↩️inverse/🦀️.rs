//! ↩️ Inverse for `CreatePvSystem` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::CreatePvSystem, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::CreatePvSystem> + From<crate::DeletePvSystem>,
{
    Ok((|| {
    if base.model.pv_systems.iter().any(|item| item.id == payload.id) || payload.index as usize > base.model.pv_systems.len() {
        return Vec::new();
    }
    vec![vocabulary::delete_pv_system(payload.id)]

    })())
}
//#endregion 🔖️Inverse

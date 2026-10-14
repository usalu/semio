//! ↩️ Inverse for `DeleteShwSystem` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::DeleteShwSystem, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::DeleteShwSystem> + From<crate::CreateShwSystem>,
{
    Ok((|| {
    let Some(index) = base.model.shw_systems.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let existing = &base.model.shw_systems[index];
    vec![vocabulary::create_shw_system(index as u32, existing.id, existing.heater_capacity_w, existing.storage_volume_m3, existing.setpoint_c, existing.schedule_id)]

    })())
}
//#endregion 🔖️Inverse

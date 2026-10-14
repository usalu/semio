//! ↩️ Inverse for `DeletePvSystem` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::DeletePvSystem, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::DeletePvSystem> + From<crate::CreatePvSystem>,
{
    Ok((|| {
    let Some(index) = base.model.pv_systems.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if base.model.electrical_load_centers.iter().any(|centre| centre.pv_ids.contains(&payload.id)) {
        return Vec::new();
    }
    let existing = &base.model.pv_systems[index];
    vec![vocabulary::create_pv_system(index as u32, existing.id, existing.dc_capacity_w, existing.area_m2, existing.tilt_deg, existing.azimuth_deg, existing.module_efficiency, existing.inverter_efficiency)]

    })())
}
//#endregion 🔖️Inverse

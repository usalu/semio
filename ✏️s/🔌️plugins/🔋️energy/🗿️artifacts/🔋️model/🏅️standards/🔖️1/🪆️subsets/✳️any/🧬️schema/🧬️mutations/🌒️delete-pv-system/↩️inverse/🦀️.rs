//! ↩️ Inverse for `DeletePvSystem` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeletePvSystem, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(index) = base.model.pv_systems.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if base.model.electrical_load_centers.iter().any(|centre| centre.pv_ids.contains(&payload.id)) {
        return Vec::new();
    }
    let existing = &base.model.pv_systems[index];
    vec![vocabulary::create_pv_system(index as u32, existing.id, existing.dc_capacity_w, existing.area_m2, existing.tilt_deg, existing.azimuth_deg, existing.module_efficiency, existing.inverter_efficiency)]
}
//#endregion 🔖️Inverse

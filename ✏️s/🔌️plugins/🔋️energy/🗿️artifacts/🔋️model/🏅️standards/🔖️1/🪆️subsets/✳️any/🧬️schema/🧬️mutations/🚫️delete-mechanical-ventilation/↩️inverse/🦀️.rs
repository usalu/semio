//! ↩️ Inverse for `DeleteMechanicalVentilation` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteMechanicalVentilation, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(index) = base.model.mechanical_ventilations.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let existing = &base.model.mechanical_ventilations[index];
    vec![vocabulary::create_mechanical_ventilation(index as u32, existing.id, existing.zone_id, existing.schedule_id, existing.design_flow_m3_s, existing.fan_total_efficiency, existing.fan_delta_pressure_pa)]
}
//#endregion 🔖️Inverse

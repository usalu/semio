//! ↩️ Inverse for `DeleteInfiltration` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteInfiltration, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(index) = base.model.infiltrations.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let existing = &base.model.infiltrations[index];
    vec![vocabulary::create_infiltration(index as u32, existing.id, existing.zone_id, existing.schedule_id, existing.method, existing.design_flow_ach, existing.flow_per_exterior_area_m3_s_m2, existing.effective_leakage_area_m2, existing.discharge_coefficient, existing.stack_height_m, existing.constant_term_coefficient, existing.temperature_term_coefficient, existing.velocity_term_coefficient, existing.velocity_squared_term_coefficient)]
}
//#endregion 🔖️Inverse

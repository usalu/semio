//! ↩️ Inverse for `ChangeMechanicalVentilationDesignFlow` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeMechanicalVentilationDesignFlow, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.mechanical_ventilations.iter().find(|item| item.id == payload.id) {
        Some(item) if item.design_flow_m3_s != payload.new_design_flow_m3_s && !(!payload.new_design_flow_m3_s.is_finite() || payload.new_design_flow_m3_s < 0.0) => vec![vocabulary::change_mechanical_ventilation_design_flow(payload.id, item.design_flow_m3_s)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

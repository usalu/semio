//! ↩️ Inverse for `ChangeInfiltrationFlowPerExteriorArea` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeInfiltrationFlowPerExteriorArea, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.infiltrations.iter().find(|item| item.id == payload.id) {
        Some(item) if item.flow_per_exterior_area_m3_s_m2 != payload.new_flow_per_exterior_area_m3_s_m2 && !(!payload.new_flow_per_exterior_area_m3_s_m2.is_finite() || payload.new_flow_per_exterior_area_m3_s_m2 < 0.0) => vec![vocabulary::change_infiltration_flow_per_exterior_area(payload.id, item.flow_per_exterior_area_m3_s_m2)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

//! ↩️ Inverse for `ChangeAirLoopDesignSupplyAirFlow` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeAirLoopDesignSupplyAirFlow, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.air_loops.iter().find(|item| item.id == payload.id) {
        Some(item) if item.design_supply_air_flow_m3_s != payload.new_design_supply_air_flow_m3_s && !(!payload.new_design_supply_air_flow_m3_s.is_finite() || payload.new_design_supply_air_flow_m3_s <= 0.0) => {
            vec![vocabulary::change_air_loop_design_supply_air_flow(payload.id, item.design_supply_air_flow_m3_s)]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

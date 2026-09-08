//! ↩️ Inverse for `DeleteAirLoop` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteAirLoop, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.air_loops.iter().find(|item| item.id == payload.id) {
        Some(item) if !(base.model.outdoor_air_systems.iter().any(|system| system.air_loop_id == payload.id)) => vec![vocabulary::create_air_loop(item.id, item.name.clone(), item.supply_node_id, item.return_node_id, item.design_supply_air_flow_m3_s, item.terminal_zone_ids.clone())],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

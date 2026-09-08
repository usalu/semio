//! ↩️ Inverse for `ChangeAirLoopSupplyNode` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeAirLoopSupplyNode, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.air_loops.iter().find(|item| item.id == payload.id) {
        Some(item) if item.supply_node_id != payload.new_supply_node_id && !(payload.new_supply_node_id == 0) => vec![vocabulary::change_air_loop_supply_node(payload.id, item.supply_node_id)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

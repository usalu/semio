//! ↩️ Inverse for `ChangeAirLoopReturnNode` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeAirLoopReturnNode, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.air_loops.iter().find(|item| item.id == payload.id) {
        Some(item) if item.return_node_id != payload.new_return_node_id && !(payload.new_return_node_id == 0) => vec![vocabulary::change_air_loop_return_node(payload.id, item.return_node_id)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

//! ↩️ Inverse for `ChangeAirLoopReturnNode` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeAirLoopReturnNode, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeAirLoopReturnNode>,
{
    Ok((|| {
    match base.model.air_loops.iter().find(|item| item.id == payload.id) {
        Some(item) if item.return_node_id != payload.new_return_node_id && !(payload.new_return_node_id == 0) => vec![vocabulary::change_air_loop_return_node(payload.id, item.return_node_id)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse

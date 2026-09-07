//! ↩️ Inverse for `RemoveAirLoopTerminalZone` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::RemoveAirLoopTerminalZone, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.air_loops.iter().find(|item| item.id == payload.id) {
        Some(existing) if !(!existing.terminal_zone_ids.contains(&payload.zone_id)) => vec![vocabulary::add_air_loop_terminal_zone(payload.id, payload.zone_id)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

//! ↩️ Inverse for `AddAirLoopTerminalZone` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::AddAirLoopTerminalZone, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.air_loops.iter().find(|item| item.id == payload.id) {
        Some(existing) if !((!base.model.zones.iter().any(|zone| zone.id == payload.zone_id)) || (existing.terminal_zone_ids.contains(&payload.zone_id))) => vec![vocabulary::remove_air_loop_terminal_zone(payload.id, payload.zone_id)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse

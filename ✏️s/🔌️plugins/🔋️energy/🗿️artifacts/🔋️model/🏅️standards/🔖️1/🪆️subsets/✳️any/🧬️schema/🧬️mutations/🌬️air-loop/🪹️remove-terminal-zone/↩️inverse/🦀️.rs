//! ↩️ Inverse for `RemoveAirLoopTerminalZone` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::RemoveAirLoopTerminalZone, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.model.air_loops.iter().find(|item| item.id == payload.id) {
        Some(existing) if !(!existing.terminal_zone_ids.contains(&payload.zone_id)) => vec![vocabulary::add_air_loop_terminal_zone(payload.id, payload.zone_id, existing.terminal_zone_ids.iter().position(|entry| *entry == payload.zone_id).map(|index| index as u32))],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse

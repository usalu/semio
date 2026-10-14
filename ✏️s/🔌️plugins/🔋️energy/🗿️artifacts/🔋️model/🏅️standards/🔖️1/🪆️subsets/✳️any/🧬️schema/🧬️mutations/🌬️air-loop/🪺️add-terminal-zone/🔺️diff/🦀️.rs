//! 🔺️ Sparse diff builder for `AddAirLoopTerminalZone` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ListEdit, ModelAirLoopPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::AddAirLoopTerminalZone, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.air_loops.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Air loop {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.zones.iter().any(|zone| zone.id == payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.zone_id.0), [payload.zone_id.0.to_string()]);
    }
    if existing.terminal_zone_ids.contains(&payload.zone_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Air loop {} already lists terminal zone {}.", payload.id.0, payload.zone_id.0), [payload.zone_id.0.to_string()]);
    }
    let position = payload.index.map_or_else(|| existing.terminal_zone_ids.iter().position(|entry| entry.0 > payload.zone_id.0).unwrap_or(existing.terminal_zone_ids.len()), |index| index as usize);
    if position > existing.terminal_zone_ids.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Index {} is past the end of the list of {} terminal_zone_ids.", position, existing.terminal_zone_ids.len()), [payload.id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { air_loops: Rows::modifying(ModelAirLoopPatch { terminal_zone_ids: ListEdit::inserting(position, payload.zone_id), ..ModelAirLoopPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

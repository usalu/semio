//! 🔺️ Sparse diff builder for `RemoveAirLoopTerminalZone` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ListEdit, ModelAirLoopPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveAirLoopTerminalZone, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.air_loops.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Air loop {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !existing.terminal_zone_ids.contains(&payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Air loop {} does not list terminal zone {}.", payload.id.0, payload.zone_id.0), [payload.zone_id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { air_loops: Rows::modifying(ModelAirLoopPatch { terminal_zone_ids: ListEdit::removing_where(&existing.terminal_zone_ids, |entry| *entry == payload.zone_id), ..ModelAirLoopPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

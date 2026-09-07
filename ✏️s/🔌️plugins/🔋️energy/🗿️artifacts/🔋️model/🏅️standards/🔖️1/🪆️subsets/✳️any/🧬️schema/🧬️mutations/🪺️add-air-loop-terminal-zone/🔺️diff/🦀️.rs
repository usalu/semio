//! 🔺️ Sparse diff builder for `AddAirLoopTerminalZone` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::AddAirLoopTerminalZone, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.air_loops.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Air loop {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.zones.iter().any(|zone| zone.id == payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.zone_id.0), [payload.zone_id.0.to_string()]);
    }
    if existing.terminal_zone_ids.contains(&payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Air loop {} already lists terminal zone {}.", payload.id.0, payload.zone_id.0), [payload.zone_id.0.to_string()]);
    }
    let mut model = base.model.clone();
    if let Some(item) = model.air_loops.iter_mut().find(|item| item.id == payload.id) {
        let position = item.terminal_zone_ids.iter().position(|entry| entry.0 > payload.zone_id.0).unwrap_or(item.terminal_zone_ids.len());
        item.terminal_zone_ids.insert(position, payload.zone_id);
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

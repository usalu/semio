//! 🔺️ Sparse diff builder for `AddElectricalLoadCenterBattery` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ElectricalLoadCenterPatch, ListEdit, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::AddElectricalLoadCenterBattery, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.electrical_load_centers.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Electrical load center {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.battery_storage.iter().any(|row| row.id == payload.battery_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Battery {} does not exist.", payload.battery_id.0), [payload.battery_id.0.to_string()]);
    }
    if payload.index as usize > existing.battery_ids.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Index {} is past the end of Electrical load center {}'s {} members.", payload.index, payload.id.0, existing.battery_ids.len()), [payload.id.0.to_string()]);
    }
    if existing.battery_ids.contains(&payload.battery_id) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Battery {} already belongs to Electrical load center {}.", payload.battery_id.0, payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { electrical_load_centers: Rows::modifying(ElectricalLoadCenterPatch { battery_ids: ListEdit::inserting(payload.index as usize, payload.battery_id), ..ElectricalLoadCenterPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

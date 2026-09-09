//! 🔺️ Sparse diff builder for `AddElectricalLoadCenterBattery` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
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
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of Electrical load center {}'s {} members.", payload.index, payload.id.0, existing.battery_ids.len()), [payload.id.0.to_string()]);
    }
    if existing.battery_ids.contains(&payload.battery_id) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Battery {} already belongs to Electrical load center {}.", payload.battery_id.0, payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.electrical_load_centers.iter_mut().find(|item| item.id == payload.id) {
        item.battery_ids.insert(payload.index as usize, payload.battery_id);
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

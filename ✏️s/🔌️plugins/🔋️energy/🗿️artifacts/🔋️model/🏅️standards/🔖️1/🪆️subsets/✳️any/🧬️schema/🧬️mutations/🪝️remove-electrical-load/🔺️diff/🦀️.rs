//! 🔺️ Sparse diff builder for `RemoveElectricalLoadCenterBattery` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveElectricalLoadCenterBattery, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.electrical_load_centers.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Electrical load center {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !existing.battery_ids.contains(&payload.battery_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Battery {} is not a member of Electrical load center {}.", payload.battery_id.0, payload.id.0), [payload.battery_id.0.to_string()]);
    }
    let mut model = base.model.clone();
    if let Some(item) = model.electrical_load_centers.iter_mut().find(|item| item.id == payload.id) {
        item.battery_ids.retain(|candidate| *candidate != payload.battery_id);
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

//! 🔺️ Sparse diff builder for `CreateBattery` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateBattery, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.battery_storage.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Battery {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.battery_storage.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of the model's {} battery_storage.", payload.index, base.model.battery_storage.len()), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.battery_storage.insert(payload.index as usize, crate::model::BatteryAssignment { id: payload.id, capacity_kwh: payload.capacity_kwh, max_charge_w: payload.max_charge_w, max_discharge_w: payload.max_discharge_w, round_trip_efficiency: payload.round_trip_efficiency });
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

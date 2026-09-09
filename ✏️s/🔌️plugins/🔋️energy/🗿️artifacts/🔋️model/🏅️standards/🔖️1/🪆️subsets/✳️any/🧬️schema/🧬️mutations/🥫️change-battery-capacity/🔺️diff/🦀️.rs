//! 🔺️ Sparse diff builder for `ChangeBatteryCapacity` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeBatteryCapacity, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.battery_storage.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Battery {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_capacity_kwh.is_finite() || payload.new_capacity_kwh <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Battery {}: storage capacity (kWh) must be a positive finite value, got {}.", payload.id.0, payload.new_capacity_kwh), [payload.id.0.to_string()]);
    }
    if existing.capacity_kwh == payload.new_capacity_kwh {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Battery {} already carries this storage capacity (kWh): {}.", payload.id.0, payload.new_capacity_kwh));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.battery_storage.iter_mut().find(|item| item.id == payload.id) {
        item.capacity_kwh = payload.new_capacity_kwh;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

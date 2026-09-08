//! 🔺️ Sparse diff builder for `ChangeBatteryMaxDischarge` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeBatteryMaxDischarge, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.battery_storage.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Battery {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_max_discharge_w.is_finite() || payload.new_max_discharge_w <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Battery {}: maximum discharge power (W) must be a positive finite value, got {}.", payload.id.0, payload.new_max_discharge_w), [payload.id.0.to_string()]);
    }
    if existing.max_discharge_w == payload.new_max_discharge_w {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Battery {} already carries this maximum discharge power (W): {}.", payload.id.0, payload.new_max_discharge_w));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.battery_storage.iter_mut().find(|item| item.id == payload.id) {
        item.max_discharge_w = payload.new_max_discharge_w;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

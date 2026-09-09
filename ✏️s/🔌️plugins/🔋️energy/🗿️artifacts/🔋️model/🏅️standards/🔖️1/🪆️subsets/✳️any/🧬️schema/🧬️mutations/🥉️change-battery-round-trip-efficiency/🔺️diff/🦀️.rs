//! 🔺️ Sparse diff builder for `ChangeBatteryRoundTripEfficiency` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeBatteryRoundTripEfficiency, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.battery_storage.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Battery {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(payload.new_round_trip_efficiency > 0.0 && payload.new_round_trip_efficiency <= 1.0) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Battery {}: round-trip efficiency must be a fraction in (0, 1], got {}.", payload.id.0, payload.new_round_trip_efficiency), [payload.id.0.to_string()]);
    }
    if existing.round_trip_efficiency == payload.new_round_trip_efficiency {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Battery {} already carries this round-trip efficiency: {}.", payload.id.0, payload.new_round_trip_efficiency));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.battery_storage.iter_mut().find(|item| item.id == payload.id) {
        item.round_trip_efficiency = payload.new_round_trip_efficiency;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

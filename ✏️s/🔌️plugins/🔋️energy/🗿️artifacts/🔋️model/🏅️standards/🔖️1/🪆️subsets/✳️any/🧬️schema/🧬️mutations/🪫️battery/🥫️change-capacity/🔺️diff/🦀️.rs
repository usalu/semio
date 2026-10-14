//! 🔺️ Sparse diff builder for `ChangeBatteryCapacity` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, BatteryAssignmentPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeBatteryCapacity, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.battery_storage.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Battery {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_capacity_kwh.is_finite() || payload.new_capacity_kwh <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Battery {}: storage capacity (kWh) must be a positive finite value, got {}.", payload.id.0, payload.new_capacity_kwh), [payload.id.0.to_string()]);
    }
    if existing.capacity_kwh == payload.new_capacity_kwh {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Battery {} already carries this storage capacity (kWh): {}.", payload.id.0, payload.new_capacity_kwh));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { battery_storage: Rows::modifying(BatteryAssignmentPatch { capacity_kwh: Some(payload.new_capacity_kwh), ..BatteryAssignmentPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

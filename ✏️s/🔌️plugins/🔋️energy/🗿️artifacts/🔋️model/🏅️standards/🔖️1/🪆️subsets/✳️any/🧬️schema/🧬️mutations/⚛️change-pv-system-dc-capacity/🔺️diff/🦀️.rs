//! 🔺️ Sparse diff builder for `ChangePvSystemDcCapacity` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePvSystemDcCapacity, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.pv_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("PV system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_dc_capacity_w.is_finite() || payload.new_dc_capacity_w <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("PV system {}: DC capacity (W) must be a positive finite value, got {}.", payload.id.0, payload.new_dc_capacity_w), [payload.id.0.to_string()]);
    }
    if existing.dc_capacity_w == payload.new_dc_capacity_w {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("PV system {} already carries this DC capacity (W): {}.", payload.id.0, payload.new_dc_capacity_w));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.pv_systems.iter_mut().find(|item| item.id == payload.id) {
        item.dc_capacity_w = payload.new_dc_capacity_w;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

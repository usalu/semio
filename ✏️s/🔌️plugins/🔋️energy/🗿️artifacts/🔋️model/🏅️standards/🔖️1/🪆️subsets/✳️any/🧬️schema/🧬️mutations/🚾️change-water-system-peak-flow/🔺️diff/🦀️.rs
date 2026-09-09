//! 🔺️ Sparse diff builder for `ChangeWaterSystemPeakFlow` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeWaterSystemPeakFlow, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.water_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Water system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_peak_flow_l_s.is_finite() || payload.new_peak_flow_l_s <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Water system {}: peak flow (L/s) must be a positive finite value, got {}.", payload.id.0, payload.new_peak_flow_l_s), [payload.id.0.to_string()]);
    }
    if existing.peak_flow_l_s == payload.new_peak_flow_l_s {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Water system {} already carries this peak flow (L/s): {}.", payload.id.0, payload.new_peak_flow_l_s));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.water_systems.iter_mut().find(|item| item.id == payload.id) {
        item.peak_flow_l_s = payload.new_peak_flow_l_s;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

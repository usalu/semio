//! 🔺️ Sparse diff builder for `CreatePvSystem` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreatePvSystem, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.pv_systems.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("PV system {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.pv_systems.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of the model's {} pv_systems.", payload.index, base.model.pv_systems.len()), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.pv_systems.insert(payload.index as usize, crate::model::PvSystemAssignment { id: payload.id, dc_capacity_w: payload.dc_capacity_w, area_m2: payload.area_m2, tilt_deg: payload.tilt_deg, azimuth_deg: payload.azimuth_deg, module_efficiency: payload.module_efficiency, inverter_efficiency: payload.inverter_efficiency });
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

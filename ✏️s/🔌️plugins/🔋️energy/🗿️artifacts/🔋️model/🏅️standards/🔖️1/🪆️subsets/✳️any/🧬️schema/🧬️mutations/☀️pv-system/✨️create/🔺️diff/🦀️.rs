//! 🔺️ Sparse diff builder for `CreatePvSystem` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch};
use crate::diff_patch::{Rows};
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreatePvSystem, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.pv_systems.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("PV system {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.pv_systems.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Index {} is past the end of the model's {} pv_systems.", payload.index, base.model.pv_systems.len()), [payload.id.0.to_string()]);
    }
    if !payload.temperature_coefficient.is_finite() || !payload.faiman_heat_loss_constant_w_m2k.is_finite() || payload.faiman_heat_loss_constant_w_m2k <= 0.0 || !payload.faiman_heat_loss_wind_w_s_m3k.is_finite() || payload.faiman_heat_loss_wind_w_s_m3k < 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "PV thermal parameters require a finite coefficient, a positive finite constant and a nonnegative finite wind coefficient.", [payload.id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { pv_systems: Rows::inserting(payload.index as usize, crate::model::PvSystemAssignment {
            id: payload.id,
            dc_capacity_w: payload.dc_capacity_w,
            area_m2: payload.area_m2,
            tilt_deg: payload.tilt_deg,
            azimuth_deg: payload.azimuth_deg,
            module_efficiency: payload.module_efficiency,
            inverter_efficiency: payload.inverter_efficiency,
            temperature_coefficient: payload.temperature_coefficient,
            faiman_heat_loss_constant_w_m2k: payload.faiman_heat_loss_constant_w_m2k,
            faiman_heat_loss_wind_w_s_m3k: payload.faiman_heat_loss_wind_w_s_m3k,
            mounting: payload.mounting,

        }), ..Default::default() }))
}
//#endregion 🔖️Diff

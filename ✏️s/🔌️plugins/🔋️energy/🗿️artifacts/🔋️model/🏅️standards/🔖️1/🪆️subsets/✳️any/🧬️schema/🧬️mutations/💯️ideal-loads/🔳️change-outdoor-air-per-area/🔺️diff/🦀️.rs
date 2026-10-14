//! 🔺️ Sparse diff builder for `ChangeIdealLoadsSystemOutdoorAirPerArea` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, IdealLoadsSystemPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeIdealLoadsSystemOutdoorAirPerArea, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.ideal_loads.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Ideal loads system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_outdoor_air_per_area_m3_s_m2.is_finite() || payload.new_outdoor_air_per_area_m3_s_m2 < 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("An outdoor air rate per floor area must be a non-negative finite number, got {}.", payload.new_outdoor_air_per_area_m3_s_m2), [payload.id.0.to_string()]);
    }
    if existing.outdoor_air_per_area_m3_s_m2 == payload.new_outdoor_air_per_area_m3_s_m2 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Ideal loads system {} already has that outdoor air rate per floor area.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { ideal_loads: Rows::modifying(IdealLoadsSystemPatch { outdoor_air_per_area_m3_s_m2: Some(payload.new_outdoor_air_per_area_m3_s_m2), ..IdealLoadsSystemPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

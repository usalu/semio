//! 🔺️ Sparse diff builder for `ChangeIdealLoadsSystemOutdoorAirPerPerson` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, IdealLoadsSystemPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeIdealLoadsSystemOutdoorAirPerPerson, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.ideal_loads.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Ideal loads system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_outdoor_air_per_person_m3_s.is_finite() || payload.new_outdoor_air_per_person_m3_s < 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("An outdoor air rate per person must be a non-negative finite number, got {}.", payload.new_outdoor_air_per_person_m3_s), [payload.id.0.to_string()]);
    }
    if existing.outdoor_air_per_person_m3_s == payload.new_outdoor_air_per_person_m3_s {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Ideal loads system {} already has that outdoor air rate per person.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { ideal_loads: Rows::modifying(IdealLoadsSystemPatch { outdoor_air_per_person_m3_s: Some(payload.new_outdoor_air_per_person_m3_s), ..IdealLoadsSystemPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

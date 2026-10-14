//! 🔺️ Sparse diff builder for `ChangeIdealLoadsSystemMinCoolingSupplyAirTemp` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, IdealLoadsSystemPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeIdealLoadsSystemMinCoolingSupplyAirTemp, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.ideal_loads.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Ideal loads system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_min_cooling_supply_air_temp_c.is_finite() || !(-100.0..=200.0).contains(&payload.new_min_cooling_supply_air_temp_c) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A cooling supply air temperature must lie between -100.0 and 200.0, got {}.", payload.new_min_cooling_supply_air_temp_c), [payload.id.0.to_string()]);
    }
    if existing.min_cooling_supply_air_temp_c == payload.new_min_cooling_supply_air_temp_c {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Ideal loads system {} already has that minimum cooling supply air temperature.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { ideal_loads: Rows::modifying(IdealLoadsSystemPatch { min_cooling_supply_air_temp_c: Some(payload.new_min_cooling_supply_air_temp_c), ..IdealLoadsSystemPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff

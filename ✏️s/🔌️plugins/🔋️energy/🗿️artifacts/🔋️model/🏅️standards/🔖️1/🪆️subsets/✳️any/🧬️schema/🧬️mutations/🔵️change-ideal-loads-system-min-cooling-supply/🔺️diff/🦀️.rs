//! 🔺️ Sparse diff builder for `ChangeIdealLoadsSystemMinCoolingSupplyAirTemp` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeIdealLoadsSystemMinCoolingSupplyAirTemp, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.ideal_loads.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Ideal loads system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_min_cooling_supply_air_temp_c.is_finite() || !(-100.0..=200.0).contains(&payload.new_min_cooling_supply_air_temp_c) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A cooling supply air temperature must lie between -100.0 and 200.0, got {}.", payload.new_min_cooling_supply_air_temp_c), [payload.id.0.to_string()]);
    }
    if existing.min_cooling_supply_air_temp_c == payload.new_min_cooling_supply_air_temp_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Ideal loads system {} already has that minimum cooling supply air temperature.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.ideal_loads.iter_mut().find(|item| item.id == payload.id) {
        item.min_cooling_supply_air_temp_c = payload.new_min_cooling_supply_air_temp_c;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

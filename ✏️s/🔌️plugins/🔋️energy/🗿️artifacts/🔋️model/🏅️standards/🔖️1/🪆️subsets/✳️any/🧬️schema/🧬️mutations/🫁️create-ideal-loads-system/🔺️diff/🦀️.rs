//! 🔺️ Sparse diff builder for `CreateIdealLoadsSystem` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateIdealLoadsSystem, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.ideal_loads.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Ideal loads system {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if !base.model.zones.iter().any(|zone| zone.id == payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.zone_id.0), [payload.zone_id.0.to_string()]);
    }
    if !payload.max_heating_supply_air_temp_c.is_finite() || !(-100.0..=200.0).contains(&payload.max_heating_supply_air_temp_c) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A heating supply air temperature must lie between -100.0 and 200.0, got {}.", payload.max_heating_supply_air_temp_c), [payload.id.0.to_string()]);
    }
    if !payload.min_cooling_supply_air_temp_c.is_finite() || !(-100.0..=200.0).contains(&payload.min_cooling_supply_air_temp_c) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A cooling supply air temperature must lie between -100.0 and 200.0, got {}.", payload.min_cooling_supply_air_temp_c), [payload.id.0.to_string()]);
    }
    if !payload.outdoor_air_per_person_m3_s.is_finite() || payload.outdoor_air_per_person_m3_s < 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("An outdoor air rate per person must be a non-negative finite number, got {}.", payload.outdoor_air_per_person_m3_s), [payload.id.0.to_string()]);
    }
    if !payload.outdoor_air_per_area_m3_s_m2.is_finite() || payload.outdoor_air_per_area_m3_s_m2 < 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("An outdoor air rate per floor area must be a non-negative finite number, got {}.", payload.outdoor_air_per_area_m3_s_m2), [payload.id.0.to_string()]);
    }
    if !payload.max_heating_capacity_present && payload.max_heating_capacity_w != 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", "An absent heating capacity carries the value zero.".to_string(), [payload.id.0.to_string()]);
    }
    if !payload.max_cooling_capacity_present && payload.max_cooling_capacity_w != 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", "An absent cooling capacity carries the value zero.".to_string(), [payload.id.0.to_string()]);
    }
    if payload.max_heating_capacity_present && (!payload.max_heating_capacity_w.is_finite() || payload.max_heating_capacity_w <= 0.0) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A stated heating capacity must be a positive finite number, got {}.", payload.max_heating_capacity_w), [payload.id.0.to_string()]);
    }
    if payload.max_cooling_capacity_present && (!payload.max_cooling_capacity_w.is_finite() || payload.max_cooling_capacity_w <= 0.0) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A stated cooling capacity must be a positive finite number, got {}.", payload.max_cooling_capacity_w), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.ideal_loads.push(crate::model::IdealLoadsSystem { id: payload.id, zone_id: payload.zone_id, max_heating_supply_air_temp_c: payload.max_heating_supply_air_temp_c, min_cooling_supply_air_temp_c: payload.min_cooling_supply_air_temp_c, max_heating_capacity_w: payload.max_heating_capacity_present.then_some(payload.max_heating_capacity_w), max_cooling_capacity_w: payload.max_cooling_capacity_present.then_some(payload.max_cooling_capacity_w), outdoor_air_per_person_m3_s: payload.outdoor_air_per_person_m3_s, outdoor_air_per_area_m3_s_m2: payload.outdoor_air_per_area_m3_s_m2 });
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

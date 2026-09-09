use super::*;

fn unlimited_system() -> IdealLoadsConfig {
    IdealLoadsConfig { max_heating_supply_air_temp_c: 50.0, min_cooling_supply_air_temp_c: 13.0, max_heating_capacity_w: None, max_cooling_capacity_w: None, outdoor_air_per_person_m3_s: 0.01, outdoor_air_per_area_m3_s_m2: 0.0 }
}

#[test]
fn heating_meets_demand() {
    let system = unlimited_system();
    let input = IdealLoadsInput {
        zone_temp_c: 18.0,
        zone_humidity_ratio: 0.008,
        outdoor_temp_c: 5.0,
        outdoor_humidity_ratio: 0.004,
        heating_setpoint_c: 21.0,
        cooling_setpoint_c: 24.0,
        zone_heating_demand_w: 3000.0,
        zone_cooling_demand_w: 0.0,
        occupancy: 2.0,
        floor_area_m2: 50.0,
    };
    let out = ideal_loads_deliver(&input, &system);
    assert!((out.sensible_heating_w - 3000.0).abs() < 1e-6);
    assert_eq!(out.unmet_heating_w, 0.0);
    assert!(out.sensible_delivered_w > 0.0);
}

#[test]
fn capacity_limits_cooling() {
    let system = IdealLoadsConfig { max_cooling_capacity_w: Some(1000.0), ..unlimited_system() };
    let input = IdealLoadsInput {
        zone_temp_c: 30.0,
        zone_humidity_ratio: 0.01,
        outdoor_temp_c: 35.0,
        outdoor_humidity_ratio: 0.015,
        heating_setpoint_c: 21.0,
        cooling_setpoint_c: 24.0,
        zone_heating_demand_w: 0.0,
        zone_cooling_demand_w: 5000.0,
        occupancy: 1.0,
        floor_area_m2: 40.0,
    };
    let out = ideal_loads_deliver(&input, &system);
    assert!((out.sensible_cooling_w - 1000.0).abs() < 1e-6);
    assert!((out.unmet_cooling_w - 4000.0).abs() < 1e-6);
}

#[test]
fn economizer_active_when_oa_cooler() {
    let system = unlimited_system();
    let input = IdealLoadsInput {
        zone_temp_c: 25.0,
        zone_humidity_ratio: 0.01,
        outdoor_temp_c: 15.0,
        outdoor_humidity_ratio: 0.006,
        heating_setpoint_c: 21.0,
        cooling_setpoint_c: 24.0,
        zone_heating_demand_w: 0.0,
        zone_cooling_demand_w: 2000.0,
        occupancy: 1.0,
        floor_area_m2: 30.0,
    };
    let out = ideal_loads_deliver_with_controls(&input, &system, EconomizerControl::DifferentialDryBulb, HumidityControl::None);
    assert!(out.economizer_active);
}

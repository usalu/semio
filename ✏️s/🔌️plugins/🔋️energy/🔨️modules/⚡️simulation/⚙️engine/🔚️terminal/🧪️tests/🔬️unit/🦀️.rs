use super::*;

#[test]
fn cav_constant_flow() {
    let term = AirTerminal::Cav { max_flow_m3_s: 0.3 };
    let req = TerminalRequest {
        supply_temperature_c: 13.0,
        supply_humidity_ratio: 0.008,
        zone_temperature_c: 22.0,
        zone_humidity_ratio: 0.01,
        heating_load_w: 0.0,
        cooling_load_w: 2000.0,
        pressure_pa: 101_325.0,
        damper_position: 1.0,
        hot_duct_temp_c: 35.0,
        cold_duct_temp_c: 13.0,
    };
    let out = term.simulate(&req);
    assert!((out.mass_flow_kg_s - 0.36).abs() < 0.05);
    assert!((out.discharge_temperature_c - 13.0).abs() < 1e-6);
}

#[test]
fn vav_reheat_adds_heat() {
    let term = AirTerminal::VavReheat { min_flow_m3_s: 0.05, max_flow_m3_s: 0.4, reheat: HeatingCoil::Electric { capacity_w: 5000.0, efficiency: 1.0 } };
    let req = TerminalRequest {
        supply_temperature_c: 13.0,
        supply_humidity_ratio: 0.008,
        zone_temperature_c: 20.0,
        zone_humidity_ratio: 0.009,
        heating_load_w: 2000.0,
        cooling_load_w: 0.0,
        pressure_pa: 101_325.0,
        damper_position: 0.5,
        hot_duct_temp_c: 35.0,
        cold_duct_temp_c: 13.0,
    };
    let out = term.simulate(&req);
    assert!(out.reheat_w > 0.0);
    assert!(out.discharge_temperature_c > req.supply_temperature_c);
}

#[test]
fn dual_duct_mixes_temperatures() {
    let term = AirTerminal::DualDuct { hot_max_m3_s: 0.2, cold_max_m3_s: 0.3, mixing_damper: 0.5 };
    let req = TerminalRequest {
        supply_temperature_c: 15.0,
        supply_humidity_ratio: 0.009,
        zone_temperature_c: 22.0,
        zone_humidity_ratio: 0.01,
        heating_load_w: 0.0,
        cooling_load_w: 0.0,
        pressure_pa: 101_325.0,
        damper_position: 0.5,
        hot_duct_temp_c: 40.0,
        cold_duct_temp_c: 12.0,
    };
    let out = term.simulate(&req);
    assert!(out.discharge_temperature_c > req.cold_duct_temp_c);
    assert!(out.discharge_temperature_c < req.hot_duct_temp_c);
}

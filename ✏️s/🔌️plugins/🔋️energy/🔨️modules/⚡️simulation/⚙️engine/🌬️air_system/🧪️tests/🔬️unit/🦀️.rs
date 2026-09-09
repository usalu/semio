use super::*;
use crate::curves::PerformanceCurve;
use crate::fans::FanType;
use crate::props::humidity_ratio_from_rh;
use crate::units::P_STD;

fn test_cooling() -> CoolingCoil {
    CoolingCoil::DxSingleSpeed { rated_capacity_w: 50_000.0, rated_shr: 0.75, cop_curve: PerformanceCurve::Constant(1.0) }
}

fn test_fan() -> Fan {
    Fan {
        fan_type: FanType::VariableVolume,
        max_flow_m3_s: 5.0,
        max_pressure_rise_pa: 800.0,
        motor_efficiency: 0.9,
        pressure_curve: PerformanceCurve::Constant(1.0),
        efficiency_curve: PerformanceCurve::Constant(0.65),
        part_load_curve: PerformanceCurve::Cubic { coeffs: [0.0, 0.2, 0.5, 0.3] },
    }
}

#[test]
fn vav_system_cools_mixed_air() {
    let system = AirSystem::Vav { supply_fan: test_fan(), return_fan: None, cooling: test_cooling(), heating: None, min_flow_m3_s: 1.0, max_flow_m3_s: 4.0 };
    let req = AirSystemRequest {
        outdoor_temperature_c: 32.0,
        outdoor_humidity_ratio: humidity_ratio_from_rh(32.0, 0.5, P_STD),
        outdoor_pressure_pa: P_STD,
        return_temperature_c: 24.0,
        return_humidity_ratio: 0.01,
        total_cooling_load_w: 30_000.0,
        total_heating_load_w: 0.0,
        oa_fraction: 0.2,
        economizer: EconomizerControl::None,
        zone_terminals: vec![],
        requested_supply_flow_m3_s: 3.0,
    };
    let out = simulate_air_system(&system, &req);
    assert!(out.supply_temperature_c < out.mixed_air_temperature_c);
    assert!(out.total_cooling_w > 0.0);
    assert!(out.supply_fan_power_w > 0.0);
}

#[test]
fn economizer_detected_when_oa_cooler() {
    let system = AirSystem::Cav { supply_fan: test_fan(), return_fan: None, cooling: test_cooling(), heating: None, design_flow_m3_s: 2.0 };
    let req = AirSystemRequest {
        outdoor_temperature_c: 15.0,
        outdoor_humidity_ratio: 0.006,
        outdoor_pressure_pa: P_STD,
        return_temperature_c: 24.0,
        return_humidity_ratio: 0.01,
        total_cooling_load_w: 5000.0,
        total_heating_load_w: 0.0,
        oa_fraction: 0.15,
        economizer: EconomizerControl::DifferentialDryBulb,
        zone_terminals: vec![],
        requested_supply_flow_m3_s: 2.0,
    };
    let out = simulate_air_system(&system, &req);
    assert!(out.economizer_active);
}

#[test]
fn doas_conditions_outdoor_air() {
    let system = AirSystem::Doas { supply_fan: test_fan(), cooling: test_cooling(), heating: Some(HeatingCoil::Electric { capacity_w: 10_000.0, efficiency: 1.0 }), erv_effectiveness: 0.7, design_oa_m3_s: 0.5 };
    let req = AirSystemRequest {
        outdoor_temperature_c: 30.0,
        outdoor_humidity_ratio: humidity_ratio_from_rh(30.0, 0.6, P_STD),
        outdoor_pressure_pa: P_STD,
        return_temperature_c: 22.0,
        return_humidity_ratio: 0.009,
        total_cooling_load_w: 8000.0,
        total_heating_load_w: 0.0,
        oa_fraction: 1.0,
        economizer: EconomizerControl::None,
        zone_terminals: vec![],
        requested_supply_flow_m3_s: 0.5,
    };
    let out = simulate_air_system(&system, &req);
    assert!(out.mixed_air_temperature_c < req.outdoor_temperature_c);
    assert!((out.outdoor_air_mass_flow_kg_s - out.supply_mass_flow_kg_s).abs() < 0.01);
}

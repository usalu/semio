
use super::*;
use crate::curves::PerformanceCurve;
use crate::fans::FanType;
use crate::units::P_STD;

#[test]
fn baseboard_delivers_heat() {
    let eq = ZoneEquipment::Baseboard { heating: HeatingCoil::Electric { capacity_w: 5000.0, efficiency: 1.0 } };
    let req = ZoneEquipmentRequest {
        zone_temperature_c: 18.0,
        zone_humidity_ratio: 0.008,
        heating_load_w: 3000.0,
        cooling_load_w: 0.0,
        outdoor_temperature_c: 5.0,
        outdoor_humidity_ratio: 0.005,
        outdoor_pressure_pa: P_STD,
        supply_air_temp_c: 20.0,
        supply_air_humidity_ratio: 0.008,
        supply_mass_flow_kg_s: 0.0,
    };
    let out = eq.simulate(&req);
    assert!(out.delivered_heating_w > 0.0);
}

#[test]
fn vrf_respects_capacity() {
    let eq = ZoneEquipment::VrfTerminal { heating_cap_w: 2000.0, cooling_cap_w: 2500.0, cop_heating: 3.5, cop_cooling: 3.0 };
    let req = ZoneEquipmentRequest {
        zone_temperature_c: 24.0,
        zone_humidity_ratio: 0.01,
        heating_load_w: 0.0,
        cooling_load_w: 5000.0,
        outdoor_temperature_c: 32.0,
        outdoor_humidity_ratio: 0.015,
        outdoor_pressure_pa: P_STD,
        supply_air_temp_c: 16.0,
        supply_air_humidity_ratio: 0.009,
        supply_mass_flow_kg_s: 0.3,
    };
    let out = eq.simulate(&req);
    assert!((out.delivered_cooling_w - 2500.0).abs() < 1.0);
    assert!(out.compressor_power_w > 0.0);
}

#[test]
fn fan_coil_runs_coils_and_fan() {
    let eq = ZoneEquipment::FanCoil {
        heating: None,
        cooling: Some(CoolingCoil::DxSingleSpeed { rated_capacity_w: 8000.0, rated_shr: 0.75, cop_curve: PerformanceCurve::Constant(1.0) }),
        fan: Fan {
            fan_type: FanType::OnOff,
            max_flow_m3_s: 0.4,
            max_pressure_rise_pa: 400.0,
            motor_efficiency: 0.85,
            pressure_curve: PerformanceCurve::Constant(1.0),
            efficiency_curve: PerformanceCurve::Constant(0.6),
            part_load_curve: PerformanceCurve::Constant(1.0),
        },
        max_flow_m3_s: 0.35,
    };
    let req = ZoneEquipmentRequest {
        zone_temperature_c: 26.0,
        zone_humidity_ratio: 0.012,
        heating_load_w: 0.0,
        cooling_load_w: 4000.0,
        outdoor_temperature_c: 30.0,
        outdoor_humidity_ratio: 0.014,
        outdoor_pressure_pa: P_STD,
        supply_air_temp_c: 18.0,
        supply_air_humidity_ratio: 0.009,
        supply_mass_flow_kg_s: 0.35,
    };
    let out = eq.simulate(&req);
    assert!(out.delivered_cooling_w > 0.0);
    assert!(out.fan_power_w > 0.0);
}

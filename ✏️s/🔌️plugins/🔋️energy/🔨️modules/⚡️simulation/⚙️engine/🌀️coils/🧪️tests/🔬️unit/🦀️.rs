use super::*;

#[test]
fn electric_heating_raises_temperature() {
    let coil = HeatingCoil::Electric { capacity_w: 10_000.0, efficiency: 1.0 };
    let inlet = CoilAirState { temperature_c: 15.0, humidity_ratio: 0.008, mass_flow_kg_s: 0.5, pressure_pa: 101_325.0 };
    let out = heating_coil_output_w(&coil, &inlet, 5000.0);
    assert!(out.outlet.temperature_c > inlet.temperature_c);
    assert!((out.total_heating_w - 5000.0).abs() < 1.0);
}

#[test]
fn dx_cooling_removes_sensible_and_latent() {
    let coil = CoolingCoil::DxSingleSpeed { rated_capacity_w: 15_000.0, rated_shr: 0.75, cop_curve: PerformanceCurve::Constant(1.0) };
    let inlet = CoilAirState { temperature_c: 28.0, humidity_ratio: 0.012, mass_flow_kg_s: 0.6, pressure_pa: 101_325.0 };
    let out = cooling_coil_output_w(&coil, &inlet, 10_000.0, 0.1);
    assert!(out.outlet.temperature_c < inlet.temperature_c);
    assert!(out.sensible_cooling_w > 0.0);
    assert!(out.latent_cooling_w > 0.0);
    assert!(out.compressor_power_w > 0.0);
}

#[test]
fn bypass_factor_reduces_effect() {
    let coil = CoolingCoil::DxSingleSpeed { rated_capacity_w: 15_000.0, rated_shr: 0.8, cop_curve: PerformanceCurve::Constant(1.0) };
    let inlet = CoilAirState { temperature_c: 30.0, humidity_ratio: 0.014, mass_flow_kg_s: 0.5, pressure_pa: 101_325.0 };
    let out_low_bf = cooling_coil_output_w(&coil, &inlet, 8000.0, 0.05);
    let out_high_bf = cooling_coil_output_w(&coil, &inlet, 8000.0, 0.4);
    assert!(out_low_bf.outlet.temperature_c < out_high_bf.outlet.temperature_c);
}

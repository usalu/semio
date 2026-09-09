use super::*;
use crate::units::P_STD;

#[test]
fn ach_infiltration_scales_with_volume() {
    let spec = InfiltrationSpec {
        method: InfiltrationMethod::ScheduledAch,
        schedule_factor: 1.0,
        ach: 0.5,
        flow_per_exterior_area_m3_s_m2: 0.0,
        effective_leakage_area_m2: 0.0,
        discharge_coefficient: 0.6,
        constant_coefficient: 0.0,
        temperature_coefficient: 0.0,
        velocity_coefficient: 0.0,
        velocity_squared_coefficient: 0.0,
        stack_height_m: 3.0,
    };
    let flow = infiltration_flow_m3_s(&spec, 200.0, 50.0, 5.0, 22.0, 3.0, P_STD);
    assert!((flow - 200.0 * 0.5 / 3600.0).abs() < 1e-9);
}

#[test]
fn ventilation_load_positive_when_outdoor_colder() {
    let (sens, _) = ventilation_load_w(0.1, 22.0, 0.009, 5.0, 0.004, P_STD, 0.0);
    assert!(sens < 0.0);
}

#[test]
fn heat_recovery_reduces_load() {
    let (sens0, _) = ventilation_load_w(0.2, 22.0, 0.009, 5.0, 0.004, P_STD, 0.0);
    let (sens1, _) = ventilation_load_w(0.2, 22.0, 0.009, 5.0, 0.004, P_STD, 0.8);
    assert!(sens1.abs() < sens0.abs());
}

#[test]
fn hybrid_uses_natural_when_favorable() {
    let ctrl = HybridVentilationControl { outdoor_temp_min_c: 10.0, outdoor_temp_max_c: 28.0, max_wind_speed_m_s: 5.0, natural_ach: 2.0, mechanical_backup: true };
    let flow = hybrid_ventilation_flow_m3_s(&ctrl, 300.0, 20.0, 2.0, 0.05);
    assert!((flow - 300.0 * 2.0 / 3600.0).abs() < 1e-9);
}

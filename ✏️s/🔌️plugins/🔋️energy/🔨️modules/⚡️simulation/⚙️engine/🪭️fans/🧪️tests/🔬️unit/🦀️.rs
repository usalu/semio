use super::*;
use crate::units::RHO_AIR_REF;

fn test_fan() -> Fan {
    Fan {
        fan_type: FanType::VariableVolume,
        max_flow_m3_s: 2.0,
        max_pressure_rise_pa: 800.0,
        motor_efficiency: 0.9,
        pressure_curve: PerformanceCurve::Quadratic { coeffs: [1.0, 0.0, -0.3] },
        efficiency_curve: PerformanceCurve::Quadratic { coeffs: [0.5, 0.4, 0.1] },
        part_load_curve: PerformanceCurve::Cubic { coeffs: [0.0, 0.3, 0.5, 0.2] },
    }
}

#[test]
fn fan_laws_cubic_power() {
    assert!((fan_law_power(1000.0, 0.5) - 125.0).abs() < 1e-6);
}

#[test]
fn zero_flow_zero_power() {
    let fan = test_fan();
    let operating_point = FanOperatingPoint { volume_flow_m3_s: 0.0, pressure_rise_pa: 0.0, part_load_ratio: 0.0, speed_ratio: 0.0 };
    assert_eq!(fan_power_w(&fan, &operating_point), 0.0);
}

#[test]
fn full_load_positive_power() {
    let fan = test_fan();
    let operating_point = fan_operating_point(&fan, 2.0, 600.0);
    let p = fan_power_w(&fan, &operating_point);
    assert!(p > 0.0);
    let m_dot = fan_mass_flow_kg_s(2.0, RHO_AIR_REF);
    assert!((m_dot - 2.4).abs() < 0.1);
}

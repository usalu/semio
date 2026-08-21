//! 🌀️ Fan performance: pressure rise, efficiency curves, fan laws, and part-load power.

use crate::curves::PerformanceCurve;
use serde::{Deserialize, Serialize};

// #region 🔖️Fan
/// 🌀️ Fan type and performance specification.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fan {
    pub fan_type: FanType,
    pub max_flow_m3_s: f64,
    pub max_pressure_rise_pa: f64,
    pub motor_efficiency: f64,
    pub pressure_curve: PerformanceCurve,
    pub efficiency_curve: PerformanceCurve,
    pub part_load_curve: PerformanceCurve,
}

/// 🔧️ Fan arrangement.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FanType {
    ConstantVolume,
    VariableVolume,
    OnOff,
}

/// 📊️ Fan operating point.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FanOperatingPoint {
    pub volume_flow_m3_s: f64,
    pub pressure_rise_pa: f64,
    pub part_load_ratio: f64,
    pub speed_ratio: f64,
}
// #endregion 🔖️Fan

// #region 🔖️FanLaws
/// 📐️ Fan law scaling: Q ∝ N, ΔP ∝ N², Power ∝ N³.
pub fn fan_law_flow(base_flow_m3_s: f64, speed_ratio: f64) -> f64 {
    base_flow_m3_s * speed_ratio
}

pub fn fan_law_pressure(base_pressure_pa: f64, speed_ratio: f64) -> f64 {
    base_pressure_pa * speed_ratio * speed_ratio
}

pub fn fan_law_power(base_power_w: f64, speed_ratio: f64) -> f64 {
    base_power_w * speed_ratio.powi(3)
}

/// ⚡️ Fan shaft/electrical power [W] from flow, pressure rise, and efficiency.
pub fn fan_power_w(fan: &Fan, operating: &FanOperatingPoint) -> f64 {
    if operating.volume_flow_m3_s.abs() < 1e-9 {
        return 0.0;
    }
    let plr = operating.part_load_ratio.clamp(0.0, 1.0);
    let speed = operating.speed_ratio.clamp(0.0, 1.2);

    let design_dp = fan.max_pressure_rise_pa * fan.pressure_curve.evaluate(plr);
    let dp = if operating.pressure_rise_pa > 0.0 { operating.pressure_rise_pa } else { fan_law_pressure(design_dp, speed) };

    let flow = if operating.volume_flow_m3_s > 0.0 { operating.volume_flow_m3_s } else { fan_law_flow(fan.max_flow_m3_s * plr, speed) };

    let eta_fan = fan.efficiency_curve.evaluate(plr).clamp(0.1, 0.9);
    let eta_motor = fan.motor_efficiency.clamp(0.5, 1.0);
    let eta_total = (eta_fan * eta_motor).max(0.05);

    let hydraulic_w = flow * dp;
    let part_load_mult = fan.part_load_curve.evaluate(plr).max(0.0);
    hydraulic_w / eta_total * part_load_mult
}

/// 📊️ Compute fan operating point from requested flow and system pressure.
pub fn fan_operating_point(fan: &Fan, requested_flow_m3_s: f64, system_pressure_pa: f64) -> FanOperatingPoint {
    let plr = (requested_flow_m3_s / fan.max_flow_m3_s.max(1e-6)).clamp(0.0, 1.2);
    let speed = plr.sqrt().clamp(0.0, 1.0);
    let dp_curve = fan.max_pressure_rise_pa * fan.pressure_curve.evaluate(plr);
    FanOperatingPoint { volume_flow_m3_s: requested_flow_m3_s, pressure_rise_pa: system_pressure_pa.max(dp_curve), part_load_ratio: plr, speed_ratio: speed }
}

/// 🌬️ Mass flow from volumetric flow and air density.
pub fn fan_mass_flow_kg_s(volume_flow_m3_s: f64, density_kg_m3: f64) -> f64 {
    volume_flow_m3_s * density_kg_m3.max(0.5)
}
// #endregion 🔖️FanLaws

#[cfg(test)]
mod tests {
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

    #[semio_framework_async_macros::async_test]
    fn fan_laws_cubic_power() {
        assert!((fan_law_power(1000.0, 0.5) - 125.0).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    fn zero_flow_zero_power() {
        let fan = test_fan();
        let operating_point = FanOperatingPoint { volume_flow_m3_s: 0.0, pressure_rise_pa: 0.0, part_load_ratio: 0.0, speed_ratio: 0.0 };
        assert_eq!(fan_power_w(&fan, &operating_point), 0.0);
    }

    #[semio_framework_async_macros::async_test]
    fn full_load_positive_power() {
        let fan = test_fan();
        let operating_point = fan_operating_point(&fan, 2.0, 600.0);
        let p = fan_power_w(&fan, &operating_point);
        assert!(p > 0.0);
        let m_dot = fan_mass_flow_kg_s(2.0, RHO_AIR_REF);
        assert!((m_dot - 2.4).abs() < 0.1);
    }
}

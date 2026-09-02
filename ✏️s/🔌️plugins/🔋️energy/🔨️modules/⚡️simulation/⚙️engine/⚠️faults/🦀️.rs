//! 🔧️ Equipment fault models: sensor offsets, fouling, dampers, refrigerant charge.

use crate::error::Severity;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

// #region 🔖️SeveritySchedule
/// 📅️ Time-varying fault severity multiplier (0 = none, 1 = full fault).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct SeveritySchedule {
    pub hourly_severity: [f64; 24],
    pub interpolation: bool,
}

impl SeveritySchedule {
    /// 📅️ Constant severity at all hours.
    pub fn constant(severity: f64) -> Self {
        Self { hourly_severity: [severity.clamp(0.0, 1.0); 24], interpolation: false }
    }

    /// 📅️ Lookup severity at hour (0–23).
    pub fn at_hour(&self, hour: u8) -> f64 {
        let h = (hour as usize).min(23);
        self.hourly_severity[h].clamp(0.0, 1.0)
    }

    /// 📅️ Interpolated severity at fractional hour.
    pub fn at_fractional_hour(&self, hour: f64) -> f64 {
        if !self.interpolation {
            return self.at_hour(hour as u8);
        }
        let h0 = (hour.floor() as usize).min(23);
        let h1 = (h0 + 1).min(23);
        let frac = hour - h0 as f64;
        let v0 = self.hourly_severity[h0];
        let v1 = self.hourly_severity[h1];
        (v0 + frac * (v1 - v0)).clamp(0.0, 1.0)
    }
}
// #endregion 🔖️SeveritySchedule

// #region 🔖️SensorOffset
/// 🌡️ Sensor bias fault on temperature or flow readings.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct SensorOffsetFault {
    pub offset: f64,
    pub unit: SensorUnit,
    pub schedule: SeveritySchedule,
    pub diagnostic_severity: Severity,
}

/// 📏️ Sensor measurement unit for offset faults.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub enum SensorUnit {
    Celsius,
    Percent,
    Pascals,
    CubicMetersPerSecond,
}

impl SensorOffsetFault {
    /// 🌡️ Apply biased reading to true value at given hour.
    pub fn biased_reading(&self, true_value: f64, hour: u8) -> f64 {
        let severity = self.schedule.at_hour(hour);
        true_value + self.offset * severity
    }

    /// 🌡️ Correct a biased reading back to true value.
    pub fn correct_reading(&self, biased_value: f64, hour: u8) -> f64 {
        let severity = self.schedule.at_hour(hour);
        biased_value - self.offset * severity
    }
}
// #endregion 🔖️SensorOffset

// #region 🔖️Fouling
/// 🦠️ Heat exchanger fouling reducing UA over time.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct FoulingFault {
    pub baseline_ua_w_per_k: f64,
    pub fouling_factor: f64,
    pub schedule: SeveritySchedule,
    pub diagnostic_severity: Severity,
}

impl FoulingFault {
    /// 🦠️ Effective UA with fouling degradation.
    pub fn effective_ua_w_per_k(&self, hour: u8) -> f64 {
        let severity = self.schedule.at_hour(hour);
        let degradation = 1.0 / (1.0 + self.fouling_factor * severity);
        self.baseline_ua_w_per_k * degradation
    }

    /// 🦠️ Additional thermal resistance from fouling [K/W].
    pub fn added_resistance_k_per_w(&self, hour: u8) -> f64 {
        let ua_clean = self.baseline_ua_w_per_k;
        let ua_fouled = self.effective_ua_w_per_k(hour);
        1.0 / ua_fouled - 1.0 / ua_clean
    }
}
// #endregion 🔖️Fouling

// #region 🔖️Damper
/// 🌬️ Damper stuck/leaking fault on air system.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub enum DamperFaultKind {
    StuckClosed,
    StuckOpen,
    Leaking { leakage_fraction: f64 },
}

/// 🌬️ Damper fault with scheduled severity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct DamperFault {
    pub kind: DamperFaultKind,
    pub design_position: f64,
    pub schedule: SeveritySchedule,
    pub diagnostic_severity: Severity,
}

impl DamperFault {
    /// 🌬️ Effective damper position (0 = closed, 1 = open).
    pub fn effective_position(&self, commanded: f64, hour: u8) -> f64 {
        let severity = self.schedule.at_hour(hour);
        let cmd = commanded.clamp(0.0, 1.0);
        match self.kind {
            DamperFaultKind::StuckClosed => cmd * (1.0 - severity),
            DamperFaultKind::StuckOpen => cmd + (1.0 - cmd) * severity,
            DamperFaultKind::Leaking { leakage_fraction } => cmd + leakage_fraction * severity * (1.0 - cmd),
        }
    }

    /// 🌬️ Airflow fraction relative to design at commanded position.
    pub fn airflow_fraction(&self, commanded: f64, hour: u8) -> f64 {
        let pos = self.effective_position(commanded, hour);
        pos.powf(0.6)
    }
}
// #endregion 🔖️Damper

// #region 🔖️RefrigerantCharge
/// ❄️ Refrigerant undercharge or overcharge fault.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub enum ChargeFaultKind {
    Undercharge,
    Overcharge,
}

/// ❄️ Refrigerant charge fault affecting capacity and power.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct RefrigerantChargeFault {
    pub kind: ChargeFaultKind,
    pub charge_deviation_fraction: f64,
    pub schedule: SeveritySchedule,
    pub diagnostic_severity: Severity,
}

impl RefrigerantChargeFault {
    /// ❄️ Capacity multiplier from charge fault.
    pub fn capacity_multiplier(&self, hour: u8) -> f64 {
        let severity = self.schedule.at_hour(hour);
        let dev = self.charge_deviation_fraction * severity;
        match self.kind {
            ChargeFaultKind::Undercharge => (1.0 - 0.5 * dev).clamp(0.3, 1.0),
            ChargeFaultKind::Overcharge => (1.0 - 0.15 * dev).clamp(0.7, 1.0),
        }
    }

    /// ❄️ Compressor power penalty multiplier.
    pub fn power_multiplier(&self, hour: u8) -> f64 {
        let severity = self.schedule.at_hour(hour);
        let dev = self.charge_deviation_fraction * severity;
        match self.kind {
            ChargeFaultKind::Undercharge => 1.0 + 0.4 * dev,
            ChargeFaultKind::Overcharge => 1.0 + 0.2 * dev,
        }
    }

    /// ❄️ Adjusted cooling output and compressor power.
    pub fn apply(&self, cooling_w: f64, compressor_w: f64, hour: u8) -> (f64, f64) {
        (cooling_w * self.capacity_multiplier(hour), compressor_w * self.power_multiplier(hour))
    }
}
// #endregion 🔖️RefrigerantCharge

// #region 🔖️FaultSet
/// 🔧️ Combined fault set for a plant or air-handling component.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct FaultSet {
    pub sensor_offsets: Vec<SensorOffsetFault>,
    pub fouling: Vec<FoulingFault>,
    pub dampers: Vec<DamperFault>,
    pub refrigerant: Vec<RefrigerantChargeFault>,
}

impl FaultSet {
    /// 🔧️ Apply all sensor offsets to a temperature reading.
    pub fn biased_temperature_c(&self, true_c: f64, hour: u8) -> f64 {
        self.sensor_offsets.iter().filter(|f| matches!(f.unit, SensorUnit::Celsius)).fold(true_c, |acc, f| f.biased_reading(acc, hour))
    }

    /// 🔧️ Worst-case fouling UA multiplier across all fouling faults.
    pub fn fouling_ua_multiplier(&self, hour: u8) -> f64 {
        if self.fouling.is_empty() {
            return 1.0;
        }
        self.fouling.iter().map(|f| f.effective_ua_w_per_k(hour) / f.baseline_ua_w_per_k).fold(1.0, f64::min)
    }
}
// #endregion 🔖️FaultSet

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_schedule_constant() {
        let sched = SeveritySchedule::constant(0.8);
        assert!((sched.at_hour(12) - 0.8).abs() < 1e-9);
    }

    #[test]
    fn sensor_offset_biases_reading() {
        let fault = SensorOffsetFault { offset: 2.0, unit: SensorUnit::Celsius, schedule: SeveritySchedule::constant(1.0), diagnostic_severity: Severity::Warning };
        assert!((fault.biased_reading(20.0, 10) - 22.0).abs() < 1e-9);
        assert!((fault.correct_reading(22.0, 10) - 20.0).abs() < 1e-9);
    }

    #[test]
    fn fouling_reduces_ua() {
        let fault = FoulingFault { baseline_ua_w_per_k: 10_000.0, fouling_factor: 0.5, schedule: SeveritySchedule::constant(1.0), diagnostic_severity: Severity::Severe };
        assert!(fault.effective_ua_w_per_k(12) < fault.baseline_ua_w_per_k);
    }

    #[test]
    fn damper_stuck_open_increases_flow() {
        let fault = DamperFault { kind: DamperFaultKind::StuckOpen, design_position: 0.5, schedule: SeveritySchedule::constant(1.0), diagnostic_severity: Severity::Warning };
        let normal = fault.airflow_fraction(0.0, 12);
        assert!(normal > 0.5);
    }

    #[test]
    fn undercharge_reduces_capacity() {
        let fault = RefrigerantChargeFault { kind: ChargeFaultKind::Undercharge, charge_deviation_fraction: 0.4, schedule: SeveritySchedule::constant(1.0), diagnostic_severity: Severity::Severe };
        assert!(fault.capacity_multiplier(8) < 1.0);
        assert!(fault.power_multiplier(8) > 1.0);
    }

    #[test]
    fn fault_set_compounds_sensor_offsets() {
        let set = FaultSet {
            sensor_offsets: vec![
                SensorOffsetFault { offset: 1.0, unit: SensorUnit::Celsius, schedule: SeveritySchedule::constant(1.0), diagnostic_severity: Severity::Warning },
                SensorOffsetFault { offset: 0.5, unit: SensorUnit::Celsius, schedule: SeveritySchedule::constant(1.0), diagnostic_severity: Severity::Warning },
            ],
            ..Default::default()
        };
        assert!((set.biased_temperature_c(20.0, 0) - 21.5).abs() < 1e-9);
    }
}

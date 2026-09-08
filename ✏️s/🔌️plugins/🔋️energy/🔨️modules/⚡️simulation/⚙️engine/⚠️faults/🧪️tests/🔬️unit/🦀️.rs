
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

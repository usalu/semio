
use super::*;

#[test]
fn constant_schedule_lookup() {
    let set = ScheduleSet { constants: vec![ConstantSchedule { id: ScheduleId(1), value: 0.5 }], ..Default::default() };
    let ctx = ScheduleContext { year: 2026, month: 1, day: 1, hour: 12, day_of_week: 4, timestep_index: 0, is_dst: false };
    assert!((set.lookup(ScheduleId(1), &ctx) - 0.5).abs() < 1e-9);
}

#[test]
fn daily_schedule_respects_limits() {
    let set = ScheduleSet { daily: vec![DailySchedule { id: ScheduleId(2), hourly_values: [2.0; 24], interpolation: ScheduleInterpolation::Discrete, limits: Some(ScheduleLimits { min: 0.0, max: 1.0 }) }], ..Default::default() };
    assert!((set.daily_value(ScheduleId(2), 10).unwrap() - 1.0).abs() < 1e-9);
}

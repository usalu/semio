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

#[test]
fn weekly_value_reads_sunday_first_slots() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/weekly-day-slots/🔣️.json")).unwrap();
    let hour = fixture["hour"].as_u64().unwrap() as u8;
    let mut daily = Vec::new();
    let mut ids = [ScheduleId(0); 7];
    for (slot, value) in fixture["dailyValueBySlot"].as_array().unwrap().iter().enumerate() {
        let id = ScheduleId((slot as u32) + 1);
        ids[slot] = id;
        daily.push(DailySchedule { id, hourly_values: [value.as_f64().unwrap(); 24], interpolation: ScheduleInterpolation::Discrete, limits: None });
    }
    let set = ScheduleSet { daily, weekly: vec![WeeklySchedule { id: ScheduleId(100), daily_schedule_ids: ids }], ..Default::default() };
    let mut seen = [false; 7];
    for case in fixture["cases"].as_array().unwrap() {
        let date = &case["date"];
        let sim = crate::calendar::SimDate::new(date["year"].as_u64().unwrap() as u16, date["month"].as_u64().unwrap() as u8, date["day"].as_u64().unwrap() as u8);
        let dow = case["dayOfWeek"].as_u64().unwrap() as u8;
        let slot = case["slot"].as_u64().unwrap() as usize;
        assert_eq!(sim.day_of_week(), dow);
        assert_eq!(weekly_day_slot(dow), slot);
        seen[slot] = true;
        let got = set.weekly_value(ScheduleId(100), dow, hour).unwrap();
        assert!((got - case["value"].as_f64().unwrap()).abs() < 1e-9);
    }
    assert!(seen.iter().all(|slot| *slot));
}

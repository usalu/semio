use super::*;

#[test]
fn leap_year_feb_has_29_days() {
    assert_eq!(days_in_month(2, true), 29);
    assert_eq!(days_in_month(2, false), 28);
}

#[test]
fn run_period_jan_week_is_168_hours() {
    let period = RunPeriod { start_month: 1, start_day: 1, end_month: 1, end_day: 7, year: 2026 };
    assert_eq!(period.total_hours(), 168);
}

#[test]
fn day_of_week_known_date() {
    let d = SimDate::new(2026, 1, 1);
    assert!(d.day_of_week() >= 1 && d.day_of_week() <= 7);
}

#[test]
fn hours_iterator_count() {
    let period = RunPeriod { start_month: 1, start_day: 1, end_month: 1, end_day: 2, year: 2026 };
    assert_eq!(period.hours().count(), 48);
}

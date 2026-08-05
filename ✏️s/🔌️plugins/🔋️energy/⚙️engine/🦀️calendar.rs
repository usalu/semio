//! 📅️ Simulation calendar: run periods, day-of-week, leap years, DST shifts.

use serde::{Deserialize, Serialize};

// #region 🔖️Date
/// 📅️ Calendar date for scheduling.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimDate {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl SimDate {
    pub const fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    /// 📅️ Day of year (1-based).
    pub fn day_of_year(&self) -> u16 {
        let days_before = days_before_month(self.month, is_leap_year(self.year));
        days_before + self.day as u16
    }

    /// 📅️ Day of week (1=Mon … 7=Sun).
    pub fn day_of_week(&self) -> u8 {
        let y = self.year as i32;
        let m = self.month as i32;
        let d = self.day as i32;
        let mm = if m < 3 { m + 12 } else { m };
        let yy = if m < 3 { y - 1 } else { y };
        let h = (d + (13 * (mm + 1)) / 5 + yy + yy / 4 - yy / 100 + yy / 400) % 7;
        match h {
            0 => 7,
            n => n as u8,
        }
    }

    /// 📅️ Advance by one day.
    pub fn advance_day(&mut self) {
        let max_day = days_in_month(self.month, is_leap_year(self.year));
        if self.day < max_day {
            self.day += 1;
            return;
        }
        self.day = 1;
        if self.month < 12 {
            self.month += 1;
        } else {
            self.month = 1;
            self.year += 1;
        }
    }
}
// #endregion 🔖️Date

// #region 🔖️RunPeriod
/// 📅️ Run period specification.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunPeriod {
    pub start_month: u8,
    pub start_day: u8,
    pub end_month: u8,
    pub end_day: u8,
    pub year: u16,
}

impl Default for RunPeriod {
    fn default() -> Self {
        Self { start_month: 1, start_day: 1, end_month: 12, end_day: 31, year: 2026 }
    }
}

impl RunPeriod {
    /// 📅️ Total simulation hours in run period.
    pub fn total_hours(&self) -> u32 {
        let mut date = SimDate::new(self.year, self.start_month, self.start_day);
        let end = SimDate::new(self.year, self.end_month, self.end_day);
        let mut hours = 0u32;
        loop {
            hours += 24;
            if date.month == end.month && date.day == end.day {
                break;
            }
            date.advance_day();
            if hours > 8760 * 2 {
                break;
            }
        }
        hours
    }

    /// 📅️ Iterator over (date, hour) pairs.
    pub fn hours(&self) -> RunPeriodHours {
        RunPeriodHours { current: SimDate::new(self.year, self.start_month, self.start_day), end: SimDate::new(self.year, self.end_month, self.end_day), hour: 0u8, index: 0u32, finished: false }
    }
}

/// 📅️ Hour iterator for a run period.
pub struct RunPeriodHours {
    current: SimDate,
    end: SimDate,
    hour: u8,
    index: u32,
    finished: bool,
}

impl RunPeriodHours {
    pub fn index(&self) -> u32 {
        self.index
    }
}

impl Iterator for RunPeriodHours {
    type Item = (SimDate, u8, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        if self.current.month > self.end.month || (self.current.month == self.end.month && self.current.day > self.end.day) {
            return None;
        }
        let item = (self.current, self.hour, self.index);
        self.index += 1;
        if self.current.month == self.end.month && self.current.day == self.end.day && self.hour == 23 {
            self.finished = true;
            return Some(item);
        }
        self.hour += 1;
        if self.hour >= 24 {
            self.hour = 0;
            self.current.advance_day();
        }
        Some(item)
    }
}
// #endregion 🔖️RunPeriod

// #region 🔖️Dst
/// 🕐️ Daylight saving time rule (simplified US-style).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DstRule {
    pub start_month: u8,
    pub start_week: u8,
    pub end_month: u8,
    pub end_week: u8,
    pub shift_hours: f64,
}

impl DstRule {
    /// 🕐️ Whether DST is active for date/hour (local standard time).
    pub fn is_dst(&self, date: SimDate, hour: u8) -> bool {
        let start_doy = nth_weekday_doy(date.year, self.start_month, self.start_week, 0);
        let end_doy = nth_weekday_doy(date.year, self.end_month, self.end_week, 0);
        let doy = date.day_of_year();
        doy >= start_doy && doy < end_doy && hour >= 2
    }

    /// 🕐️ Schedule hour shift for DST.
    pub fn schedule_shift(&self, date: SimDate, hour: u8) -> f64 {
        if self.is_dst(date, hour) {
            self.shift_hours
        } else {
            0.0
        }
    }
}
// #endregion 🔖️Dst

// #region 🔖️Helpers
fn is_leap_year(year: u16) -> bool {
    let y = year as u32;
    (y.is_multiple_of(4) && !y.is_multiple_of(100)) || y.is_multiple_of(400)
}

fn days_in_month(month: u8, leap: bool) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => 30,
    }
}

fn days_before_month(month: u8, leap: bool) -> u16 {
    let days = [0u16, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let idx = (month.saturating_sub(1)) as usize;
    let base = days.get(idx).copied().unwrap_or(0);
    if leap && month > 2 {
        base + 1
    } else {
        base
    }
}

fn nth_weekday_doy(year: u16, month: u8, nth: u8, weekday: u8) -> u16 {
    let first = SimDate::new(year, month, 1);
    let first_dow = first.day_of_week();
    let offset = (7 + weekday - first_dow) % 7;
    let day = 1 + offset + (nth.saturating_sub(1)) * 7;
    days_before_month(month, is_leap_year(year)) + day as u16
}
// #endregion 🔖️Helpers

#[cfg(test)]
mod tests {
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
}

//! 📅️ Schedule definitions and runtime lookup.

use crate::model::ScheduleId;
use serde::{Deserialize, Serialize};

// #region 🔖️ScheduleType
/// 📆️ Schedule interpolation mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScheduleInterpolation {
    Continuous,
    Discrete,
}

/// 📆️ Schedule value limit.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleLimits {
    pub min: f64,
    pub max: f64,
}

/// 📅️ Constant schedule.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConstantSchedule {
    pub id: ScheduleId,
    pub value: f64,
}

/// 📅️ Daily repeating schedule (24 hourly values).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DailySchedule {
    pub id: ScheduleId,
    pub hourly_values: [f64; 24],
    pub interpolation: ScheduleInterpolation,
    pub limits: Option<ScheduleLimits>,
}

/// 📅️ Weekly schedule (7 daily schedule ids).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WeeklySchedule {
    pub id: ScheduleId,
    pub daily_schedule_ids: [ScheduleId; 7],
}

/// 📅️ Compact rule-based annual schedule.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompactScheduleRule {
    pub start_month: u8,
    pub start_day: u8,
    pub end_month: u8,
    pub end_day: u8,
    pub daily_schedule_id: ScheduleId,
}

/// 📅️ Annual schedule with holiday overrides.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnnualSchedule {
    pub id: ScheduleId,
    pub rules: Vec<CompactScheduleRule>,
    pub default_daily_schedule_id: ScheduleId,
    pub holiday_daily_schedule_id: Option<ScheduleId>,
    pub holiday_dates: Vec<(u16, u8, u8)>,
}

/// 📅️ External time-series schedule.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesSchedule {
    pub id: ScheduleId,
    pub values: Vec<f64>,
    pub timestep_seconds: u32,
}
// #endregion 🔖️ScheduleType

// #region 🔖️ScheduleSet
/// 📚️ All schedules in a model.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ScheduleSet {
    pub constants: Vec<ConstantSchedule>,
    pub daily: Vec<DailySchedule>,
    pub weekly: Vec<WeeklySchedule>,
    pub annual: Vec<AnnualSchedule>,
    pub time_series: Vec<TimeSeriesSchedule>,
}

impl ScheduleSet {
    pub fn constant_value(&self, id: ScheduleId) -> Option<f64> {
        self.constants.iter().find(|c| c.id == id).map(|c| c.value)
    }

    pub fn daily_value(&self, id: ScheduleId, hour: u8) -> Option<f64> {
        let daily = self.daily.iter().find(|d| d.id == id)?;
        let h = (hour as usize).min(23);
        let mut v = daily.hourly_values[h];
        if let Some(limits) = daily.limits {
            v = v.clamp(limits.min, limits.max);
        }
        Some(v)
    }

    pub fn weekly_value(&self, id: ScheduleId, day_of_week: u8, hour: u8) -> Option<f64> {
        let weekly = self.weekly.iter().find(|w| w.id == id)?;
        let dow = (day_of_week as usize).min(6);
        self.daily_value(weekly.daily_schedule_ids[dow], hour)
    }

    pub fn annual_value(&self, id: ScheduleId, year: u16, month: u8, day: u8, hour: u8) -> Option<f64> {
        let annual = self.annual.iter().find(|a| a.id == id)?;
        if annual.holiday_dates.contains(&(year, month, day)) {
            if let Some(hid) = annual.holiday_daily_schedule_id {
                return self.daily_value(hid, hour);
            }
        }
        for rule in &annual.rules {
            if date_in_range(month, day, rule.start_month, rule.start_day, rule.end_month, rule.end_day) {
                return self.daily_value(rule.daily_schedule_id, hour);
            }
        }
        self.daily_value(annual.default_daily_schedule_id, hour)
    }

    pub fn lookup(&self, id: ScheduleId, ctx: &ScheduleContext) -> f64 {
        if let Some(v) = self.constant_value(id) {
            return v;
        }
        if let Some(v) = self.annual_value(id, ctx.year, ctx.month, ctx.day, ctx.hour) {
            return v;
        }
        if let Some(v) = self.weekly_value(id, ctx.day_of_week, ctx.hour) {
            return v;
        }
        if let Some(v) = self.daily_value(id, ctx.hour) {
            return v;
        }
        if let Some(ts) = self.time_series.iter().find(|t| t.id == id) {
            let idx = (ctx.timestep_index as usize).min(ts.values.len().saturating_sub(1));
            return ts.values[idx];
        }
        1.0
    }

    /// 📦️ Pre-expand schedule values for all timesteps in a run period.
    pub fn expand(&self, id: ScheduleId, ctxs: &[ScheduleContext]) -> Vec<f64> {
        ctxs.iter().map(|c| self.lookup(id, c)).collect()
    }
}
// #endregion 🔖️ScheduleSet

// #region 🔖️Context
/// 🕐️ Calendar context for schedule lookup.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleContext {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub day_of_week: u8,
    pub timestep_index: u32,
    pub is_dst: bool,
}

fn date_in_range(m: u8, d: u8, sm: u8, sd: u8, em: u8, ed: u8) -> bool {
    let md = m as u16 * 32 + d as u16;
    let start = sm as u16 * 32 + sd as u16;
    let end = em as u16 * 32 + ed as u16;
    if start <= end {
        md >= start && md <= end
    } else {
        md >= start || md <= end
    }
}
// #endregion 🔖️Context

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn constant_schedule_lookup() {
        let set = ScheduleSet { constants: vec![ConstantSchedule { id: ScheduleId(1), value: 0.5 }], ..Default::default() };
        let ctx = ScheduleContext { year: 2026, month: 1, day: 1, hour: 12, day_of_week: 4, timestep_index: 0, is_dst: false };
        assert!((set.lookup(ScheduleId(1), &ctx) - 0.5).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    fn daily_schedule_respects_limits() {
        let set = ScheduleSet { daily: vec![DailySchedule { id: ScheduleId(2), hourly_values: [2.0; 24], interpolation: ScheduleInterpolation::Discrete, limits: Some(ScheduleLimits { min: 0.0, max: 1.0 }) }], ..Default::default() };
        assert!((set.daily_value(ScheduleId(2), 10).unwrap() - 1.0).abs() < 1e-9);
    }
}

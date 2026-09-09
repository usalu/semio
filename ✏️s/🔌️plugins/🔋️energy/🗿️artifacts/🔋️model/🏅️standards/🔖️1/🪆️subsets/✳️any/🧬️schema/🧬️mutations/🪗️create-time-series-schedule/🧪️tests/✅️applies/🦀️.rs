//! 🧪️ `create-time-series-schedule` fixture — `✅️applies`: defines a new time series schedule.
//!
//! The committed `(before, mutation, after, diff, outcome)` quintet beside this file IS the
//! specification; `scenario` is the typed source it was generated from
//! (`SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test -p semio-s-plugin-energy`), and the eight law
//! assertions below read the committed bytes back, never the scenario.

use crate::mutations::fixtures::{self, snapshot, zone, Case};
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🪗️create-time-series-schedule/✅️applies/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🪗️create-time-series-schedule/✅️applies/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🪗️create-time-series-schedule/✅️applies/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🪗️create-time-series-schedule/✅️applies/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🪗️create-time-series-schedule/✅️applies/🎯️outcome/🔣️.json");

#[allow(unused_variables, unused_mut)]
fn scenario() -> (EnergyModelSnapshot, EnergyModelMutation) {
    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: crate::model::ScheduleId(1), value: 1.0 });
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: crate::model::ScheduleId(2), value: 0.5 });
    model.schedules.daily.push(crate::schedule::DailySchedule { id: crate::model::ScheduleId(10), hourly_values: [20.0; 24], interpolation: crate::schedule::ScheduleInterpolation::Continuous, limits: None });
    model.schedules.daily.push(crate::schedule::DailySchedule {
        id: crate::model::ScheduleId(11),
        hourly_values: [27.0; 24],
        interpolation: crate::schedule::ScheduleInterpolation::Discrete,
        limits: Some(crate::schedule::ScheduleLimits { min: 0.0, max: 100.0 }),
    });
    model.schedules.daily.push(crate::schedule::DailySchedule { id: crate::model::ScheduleId(12), hourly_values: [1.0; 24], interpolation: crate::schedule::ScheduleInterpolation::Continuous, limits: None });
    model.schedules.weekly.push(crate::schedule::WeeklySchedule { id: crate::model::ScheduleId(20), daily_schedule_ids: [crate::model::ScheduleId(10); 7] });
    model.schedules.annual.push(crate::schedule::AnnualSchedule {
        id: crate::model::ScheduleId(30),
        rules: vec![
            crate::schedule::CompactScheduleRule { start_month: 1, start_day: 1, end_month: 6, end_day: 30, daily_schedule_id: crate::model::ScheduleId(10) },
            crate::schedule::CompactScheduleRule { start_month: 7, start_day: 1, end_month: 12, end_day: 31, daily_schedule_id: crate::model::ScheduleId(11) },
        ],
        default_daily_schedule_id: crate::model::ScheduleId(11),
        holiday_daily_schedule_id: None,
        holiday_dates: vec![(2026, 12, 25)],
    });
    model.schedules.time_series.push(crate::schedule::TimeSeriesSchedule { id: crate::model::ScheduleId(40), values: vec![1.0, 0.5, 0.25], timestep_seconds: 3600 });
    (snapshot(model), super::create_time_series_schedule(1, crate::model::ScheduleId(41), vec![1.0, 0.9], 900))
}

fn case() -> Case {
    Case { kind: "create-time-series-schedule", directory: "🪗️create-time-series-schedule/✅️applies", before: BEFORE, after: AFTER, mutation: MUTATION, diff: DIFF, outcome: OUTCOME, scenario }
}

#[semio_framework_async_macros::async_test]
async fn writes_the_committed_vector_when_requested() {
    fixtures::write_when_requested(&case());
}

#[semio_framework_async_macros::async_test]
async fn forward_reaches_the_committed_after_snapshot() {
    fixtures::assert_forward(&case());
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_committed_before_snapshot() {
    fixtures::assert_inverse(&case());
}

#[semio_framework_async_macros::async_test]
async fn committed_documents_are_canonical() {
    fixtures::assert_canonical(&case());
}

#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    fixtures::assert_outcome(&case());
}

#[semio_framework_async_macros::async_test]
async fn produces_the_committed_diff() {
    fixtures::assert_diff(&case());
}

#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    fixtures::assert_diff_canonical(&case());
}

#[semio_framework_async_macros::async_test]
async fn committed_diff_alone_carries_before_to_after() {
    fixtures::assert_diff_applies(&case());
}

#[semio_framework_async_macros::async_test]
async fn semantic_descriptor_and_inverse_are_complete() {
    fixtures::assert_semantics(&case()).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_and_absorb_laws_hold() {
    fixtures::assert_laws(&case()).await;
}

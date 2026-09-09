//! 🧪️ `create-solar-thermal-system` fixture — `✅️applies`: inserts a new solar thermal system.
//!
//! The committed `(before, mutation, after, diff, outcome)` quintet beside this file IS the
//! specification; `scenario` is the typed source it was generated from
//! (`SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test -p semio-s-plugin-energy`), and the eight law
//! assertions below read the committed bytes back, never the scenario.

use crate::mutations::fixtures::{self, snapshot, zone, Case};
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌄️create-solar-thermal-system/✅️applies/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌄️create-solar-thermal-system/✅️applies/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌄️create-solar-thermal-system/✅️applies/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌄️create-solar-thermal-system/✅️applies/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌄️create-solar-thermal-system/✅️applies/🎯️outcome/🔣️.json");

#[allow(unused_variables, unused_mut)]
fn scenario() -> (EnergyModelSnapshot, EnergyModelMutation) {
    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.zones.push(zone(2, "ZONE TWO"));
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: crate::model::ScheduleId(1), value: 1.0 });
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: crate::model::ScheduleId(2), value: 0.5 });
    model.solar_thermal_systems.push(crate::model::SolarThermalConfig { id: crate::model::EntityId(410), collector_area_m2: 4.0, efficiency: 0.55, storage_volume_m3: 0.3, tilt_deg: 45.0, azimuth_deg: 180.0 });
    (snapshot(model), super::create_solar_thermal_system(1, crate::model::EntityId(411), 6.0, 0.6, 0.4, 40.0, 190.0))
}

fn case() -> Case {
    Case { kind: "create-solar-thermal-system", directory: "🌄️create-solar-thermal-system/✅️applies", before: BEFORE, after: AFTER, mutation: MUTATION, diff: DIFF, outcome: OUTCOME, scenario }
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

//! 🧪️ `delete-refrigeration-system` fixture — `⛔️refuses`: refuses an absent element.
//!
//! The committed `(before, mutation, after, diff, outcome)` quintet beside this file IS the
//! specification; `scenario` is the typed source it was generated from
//! (`SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test -p semio-s-plugin-energy`), and the eight law
//! assertions below read the committed bytes back, never the scenario.

use crate::mutations::fixtures::{self, snapshot, zone, Case};
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🫠️delete-refrigeration-system/⛔️refuses/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🫠️delete-refrigeration-system/⛔️refuses/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🫠️delete-refrigeration-system/⛔️refuses/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🫠️delete-refrigeration-system/⛔️refuses/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🫠️delete-refrigeration-system/⛔️refuses/🎯️outcome/🔣️.json");

#[allow(unused_variables, unused_mut)]
fn scenario() -> (EnergyModelSnapshot, EnergyModelMutation) {
    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.zones.push(zone(2, "ZONE TWO"));
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: crate::model::ScheduleId(1), value: 1.0 });
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: crate::model::ScheduleId(2), value: 0.5 });
    model.refrigeration_systems.push(crate::model::RefrigerationConfig { id: crate::model::EntityId(420), case_count: 4, design_load_w: 12000.0, defrost_schedule_id: crate::model::ScheduleId(1) });
    (snapshot(model), super::delete_refrigeration_system(crate::model::EntityId(999)))
}

fn case() -> Case {
    Case { kind: "delete-refrigeration-system", directory: "🫠️delete-refrigeration-system/⛔️refuses", before: BEFORE, after: AFTER, mutation: MUTATION, diff: DIFF, outcome: OUTCOME, scenario }
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

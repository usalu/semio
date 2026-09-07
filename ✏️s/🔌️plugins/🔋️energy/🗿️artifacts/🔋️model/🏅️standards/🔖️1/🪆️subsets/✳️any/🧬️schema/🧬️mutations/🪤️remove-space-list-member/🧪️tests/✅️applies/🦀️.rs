//! 🧪️ `remove-space-list-member` fixture — `✅️applies`: moves space list 210's membership.
//!
//! The committed `(before, mutation, after, diff, outcome)` quintet beside this file IS the
//! specification; `scenario` is the typed source it was generated from
//! (`SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test -p semio-s-plugin-energy`), and the eight law
//! assertions below read the committed bytes back, never the scenario.

use crate::artifacts::model::mutations::fixtures::{self, snapshot, zone, Case};
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

#[allow(unused_variables, unused_mut)]
fn scenario() -> (EnergyModelSnapshot, EnergyModelMutation) {
    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.zones.push(zone(2, "ZONE TWO"));
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: crate::model::ScheduleId(1), value: 1.0 });
    model.schedules.constants.push(crate::schedule::ConstantSchedule { id: crate::model::ScheduleId(2), value: 0.5 });
    model.spaces.push(crate::model::Space { id: crate::model::EntityId(200), name: "SPACE ONE".into(), zone_id: crate::model::EntityId(1), floor_area_m2: 48.0 });
    model.spaces.push(crate::model::Space { id: crate::model::EntityId(201), name: "SPACE TWO".into(), zone_id: crate::model::EntityId(2), floor_area_m2: 48.0 });
    model.space_lists.push(crate::model::SpaceList { id: crate::model::EntityId(210), name: "CORE".into(), space_ids: vec![crate::model::EntityId(200)] });
    model.space_lists.push(crate::model::SpaceList { id: crate::model::EntityId(211), name: "PERIMETER".into(), space_ids: Vec::new() });
    (snapshot(model), super::remove_space_list_member(crate::model::EntityId(210), crate::model::EntityId(200)))
}

fn case() -> Case {
    Case { kind: "remove-space-list-member", directory: "🪤️remove-space-list-member/🧪️tests/✅️applies", before: BEFORE, after: AFTER, mutation: MUTATION, diff: DIFF, outcome: OUTCOME, scenario }
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

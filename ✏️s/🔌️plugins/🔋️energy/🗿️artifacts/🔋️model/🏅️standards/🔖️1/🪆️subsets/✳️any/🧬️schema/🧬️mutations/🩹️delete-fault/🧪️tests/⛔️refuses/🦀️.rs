//! 🧪️ `delete-fault` fixture — `⛔️refuses`: refuses an absent element.
//!
//! The committed `(before, mutation, after, diff, outcome)` quintet beside this file IS the
//! specification; `scenario` is the typed source it was generated from
//! (`SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test -p semio-s-plugin-energy`), and the eight law
//! assertions below read the committed bytes back, never the scenario.

use crate::mutations::fixtures::{self, snapshot, zone, Case};
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

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
    model.ideal_loads.push(crate::model::IdealLoadsSystem {
        id: crate::model::EntityId(500),
        zone_id: crate::model::EntityId(1),
        max_heating_supply_air_temp_c: 50.0,
        min_cooling_supply_air_temp_c: 13.0,
        max_heating_capacity_w: None,
        max_cooling_capacity_w: None,
        outdoor_air_per_person_m3_s: 0.0,
        outdoor_air_per_area_m3_s_m2: 0.0,
    });
    model.ideal_loads.push(crate::model::IdealLoadsSystem {
        id: crate::model::EntityId(501),
        zone_id: crate::model::EntityId(2),
        max_heating_supply_air_temp_c: 50.0,
        min_cooling_supply_air_temp_c: 13.0,
        max_heating_capacity_w: None,
        max_cooling_capacity_w: None,
        outdoor_air_per_person_m3_s: 0.0,
        outdoor_air_per_area_m3_s_m2: 0.0,
    });
    model.faults.push(crate::model::FaultDefinition {
        id: crate::model::EntityId(440),
        target_equipment_id: crate::model::EntityId(500),
        fault_type: crate::model::FaultType::CoilFouling,
        severity: 0.3,
        start_schedule_id: crate::model::ScheduleId(1),
    });
    (snapshot(model), super::delete_fault(crate::model::EntityId(999)))
}

fn case() -> Case {
    Case { kind: "delete-fault", directory: "🩹️delete-fault/🧪️tests/⛔️refuses", before: BEFORE, after: AFTER, mutation: MUTATION, diff: DIFF, outcome: OUTCOME, scenario }
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

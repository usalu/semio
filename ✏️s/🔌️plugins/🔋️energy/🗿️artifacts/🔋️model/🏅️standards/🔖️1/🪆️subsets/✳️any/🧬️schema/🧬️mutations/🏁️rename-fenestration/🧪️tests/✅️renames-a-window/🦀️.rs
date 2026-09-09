//! 🧪️ `rename-fenestration` fixture — `✅️renames-a-window`: renames the south window.
//!
//! The committed `(before, mutation, after, diff, outcome)` quintet beside this file IS the
//! specification; `scenario` is the typed source it was generated from
//! (`SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test -p semio-s-plugin-energy`), and the eight law
//! assertions below read the committed bytes back, never the scenario.

use crate::mutations::fixtures::{self, snapshot, zone, Case};
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏁️rename-fenestration/✅️renames-a-window/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏁️rename-fenestration/✅️renames-a-window/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏁️rename-fenestration/✅️renames-a-window/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏁️rename-fenestration/✅️renames-a-window/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏁️rename-fenestration/✅️renames-a-window/🎯️outcome/🔣️.json");

#[allow(unused_variables, unused_mut)]
fn scenario() -> (EnergyModelSnapshot, EnergyModelMutation) {
    let mut model = crate::model::Model { name: "BESTEST 600".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    model.materials.push(fixtures::material(4, "PLASTERBOARD"));
    model.constructions.push(fixtures::construction(2, "LIGHTWEIGHT WALL", 4));
    model.surfaces.push(fixtures::surface(3, "WALL SOUTH", 1, 2));
    model.fenestrations.push(fixtures::window(5, "WINDOW SOUTH", 3));
    (snapshot(model), super::rename_fenestration(crate::model::EntityId(5), "WINDOW S".into()))
}

fn case() -> Case {
    Case { kind: "rename-fenestration", directory: "🏁️rename-fenestration/✅️renames-a-window", before: BEFORE, after: AFTER, mutation: MUTATION, diff: DIFF, outcome: OUTCOME, scenario }
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

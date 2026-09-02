//! 🧪️ `set-layer-locked` fixture — `locks-shape-a`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::drawing::mutations::{apply_drawing_mutation, inverse_drawing_mutation, DrawingMutation};
use crate::artifacts::drawing::schema::{find_drawing_layer, layer_base};
use crate::artifacts::drawing::DrawingSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> DrawingSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> DrawingSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> DrawingMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_drawing_mutation(&mut snapshot, &mutation()).expect("set-layer-locked applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "set-layer-locked/locks-shape-a: applied state differs from committed after-snapshot");
}

/// 🔒️ Locking is an editability flag, not a rendering one: `visible` must stay true so the layer
/// keeps drawing while it refuses edits.
#[semio_framework_async_macros::async_test]
async fn locking_does_not_hide_the_layer() {
    let base = before();
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation()).expect("set-layer-locked applies");
    let before_layer = layer_base(find_drawing_layer(&base, "shape-a").expect("before carries shape-a"));
    let after_layer = layer_base(find_drawing_layer(&snapshot, "shape-a").expect("shape-a survives a lock"));
    assert!(!before_layer.locked, "locks-shape-a's before-snapshot must start unlocked");
    assert!(after_layer.locked, "set-layer-locked must write the payload's locked flag");
    assert!(after_layer.visible, "a locked layer still renders — set-layer-locked must not touch visible");
    assert_eq!(after_layer.name, before_layer.name, "set-layer-locked must not touch the layer name");
}

/// ↩️ The inverse is a `set-layer-locked` back to the flag BASE carried.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_lock_state() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_drawing_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "set-layer-locked undoes with exactly one counter-set");
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_drawing_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "set-layer-locked/locks-shape-a: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-layer-locked/locks-shape-a: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-layer-locked/locks-shape-a: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches the diff builder: applied, and the delta is a single `locked`
/// patch on `shape-a`.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-layer-locked/locks-shape-a declares an applied outcome");
    let produced = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "set-layer-locked/locks-shape-a: the flag really flips, so no no-op warning is expected, got {:?}", produced.messages());
    let delta = produced.diff().layers.clone().expect("set-layer-locked's diff pins a layers delta");
    assert_eq!(delta.patched.len(), 1, "set-layer-locked patches exactly one layer");
    assert_eq!(delta.patched[0].patch.locked, Some(true), "the patch pins the locked field");
    assert_eq!(delta.patched[0].patch.visible, None, "set-layer-locked must not smuggle a visibility change into its patch");
}

/// 🔺️ The produced diff is EXACTLY the committed one: one `patched` entry setting `locked` alone.
/// The committed `"visible": null` beside it is the load-bearing part — locking is an editability
/// change, and a diff that also carried a visibility flag would be a different mutation.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-layer-locked/locks-shape-a: produced diff differs from the committed 🔺️diff/🔣️.json");
    let delta = outcome.diff().layers.clone().expect("set-layer-locked pins a layers delta");
    let patch = &delta.patched[0].patch;
    assert_eq!(patch.locked, Some(true), "the locked lane carries the new flag");
    assert!(patch.visible.is_none(), "locking must not smuggle a hide into the same patch");
    assert!(patch.opacity.is_none() && patch.name.is_none(), "no other base lane is written");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `DrawingDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::drawing::DrawingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-layer-locked/locks-shape-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::drawing::DrawingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::drawing::DrawingDiff as protocol::MutationDiff<DrawingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-layer-locked/locks-shape-a: committed diff did not carry before to after");
}

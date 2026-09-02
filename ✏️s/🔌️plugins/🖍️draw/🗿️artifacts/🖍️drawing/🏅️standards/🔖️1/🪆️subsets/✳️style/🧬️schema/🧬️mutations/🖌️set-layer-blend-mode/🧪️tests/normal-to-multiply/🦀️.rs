//! 🧪️ `set-layer-blend-mode` fixture — `normal-to-multiply`.
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
    apply_drawing_mutation(&mut snapshot, &mutation()).expect("set-layer-blend-mode applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "set-layer-blend-mode/normal-to-multiply: applied state differs from committed after-snapshot");
}

/// 🖌️ `blend_mode` is a free-form String in this schema, not a closed enum: the diff builder
/// compares it for equality and writes it verbatim, with no vocabulary check of its own.
#[semio_framework_async_macros::async_test]
async fn blend_mode_is_written_verbatim() {
    let base = before();
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation()).expect("set-layer-blend-mode applies");
    let before_layer = layer_base(find_drawing_layer(&base, "shape-a").expect("before carries shape-a"));
    let after_layer = layer_base(find_drawing_layer(&snapshot, "shape-a").expect("shape-a survives a blend change"));
    assert_eq!(before_layer.blend_mode, "normal", "normal-to-multiply's before-snapshot must start on normal");
    assert_eq!(after_layer.blend_mode, "multiply", "set-layer-blend-mode must write the payload's blend_mode verbatim");
    assert_eq!(after_layer.opacity, before_layer.opacity, "a blend change is not an opacity change");
}

/// ↩️ The inverse is a `set-layer-blend-mode` back to the mode BASE carried.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_blend_mode() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_drawing_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "set-layer-blend-mode undoes with exactly one counter-set");
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_drawing_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "set-layer-blend-mode/normal-to-multiply: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-layer-blend-mode/normal-to-multiply: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-layer-blend-mode/normal-to-multiply: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches the diff builder: applied, and the delta pins `blend_mode`.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-layer-blend-mode/normal-to-multiply declares an applied outcome");
    let produced = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "set-layer-blend-mode/normal-to-multiply: multiply differs from normal, so no no-op warning is expected, got {:?}", produced.messages());
    let delta = produced.diff().layers.clone().expect("set-layer-blend-mode's diff pins a layers delta");
    assert_eq!(delta.patched[0].patch.blend_mode.as_deref(), Some("multiply"), "the patch pins the blend_mode field");
}

/// 🔺️ The produced diff is EXACTLY the committed one: one `patched` entry setting `blendMode` to the
/// payload's free-form string verbatim. No vocabulary normalization happens on the way into the diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-layer-blend-mode/normal-to-multiply: produced diff differs from the committed 🔺️diff/🔣️.json");
    let delta = outcome.diff().layers.clone().expect("set-layer-blend-mode pins a layers delta");
    let patch = &delta.patched[0].patch;
    assert_eq!(patch.blend_mode.as_deref(), Some("multiply"), "the blend-mode lane carries the payload string verbatim");
    assert!(patch.opacity.is_none(), "a blend change is not an opacity change");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `DrawingDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::drawing::DrawingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-layer-blend-mode/normal-to-multiply: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::drawing::DrawingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::drawing::DrawingDiff as protocol::MutationDiff<DrawingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-layer-blend-mode/normal-to-multiply: committed diff did not carry before to after");
}

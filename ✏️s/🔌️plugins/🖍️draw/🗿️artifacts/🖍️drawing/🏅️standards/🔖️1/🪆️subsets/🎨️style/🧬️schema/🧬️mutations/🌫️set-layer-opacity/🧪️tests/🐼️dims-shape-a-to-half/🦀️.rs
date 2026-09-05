//! 🧪️ `set-layer-opacity` fixture — `🐼️dims-shape-a-to-half`.
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
    apply_drawing_mutation(&mut snapshot, &mutation()).expect("set-layer-opacity applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "set-layer-opacity/dims-shape-a-to-half: applied state differs from committed after-snapshot");
}

/// 🌫️ Opacity is an absolute f64 set, and it is independent of the fill's own alpha channel — the
/// solid fill's `color[3]` must survive at 1.0 while the layer opacity drops to 0.5.
#[semio_framework_async_macros::async_test]
async fn layer_opacity_is_independent_of_the_fill_alpha() {
    let base = before();
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation()).expect("set-layer-opacity applies");
    let before_layer = layer_base(find_drawing_layer(&base, "shape-a").expect("before carries shape-a"));
    let after_layer = layer_base(find_drawing_layer(&snapshot, "shape-a").expect("shape-a survives a dim"));
    assert_eq!(before_layer.opacity, 1.0, "dims-shape-a-to-half's before-snapshot must start fully opaque");
    assert_eq!(after_layer.opacity, 0.5, "set-layer-opacity must write the payload's opacity verbatim");
    assert_eq!(after_layer.attributes.fill, before_layer.attributes.fill, "dimming a layer must not rewrite its fill alpha");
}

/// ↩️ The inverse is a `set-layer-opacity` back to the value BASE carried.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_opacity() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_drawing_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "set-layer-opacity undoes with exactly one counter-set");
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_drawing_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "set-layer-opacity/dims-shape-a-to-half: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-layer-opacity/dims-shape-a-to-half: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-layer-opacity/dims-shape-a-to-half: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches the diff builder. `set-layer-opacity` guards a
/// `mutation.invariant` Fatal for non-finite values before its no-op check; 0.5 is finite and
/// different, so the outcome must be clean.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-layer-opacity/dims-shape-a-to-half declares an applied outcome");
    let produced = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "set-layer-opacity/dims-shape-a-to-half: 0.5 is finite and differs from 1.0, so neither the invariant nor the no-op guard may fire, got {:?}", produced.messages());
    let delta = produced.diff().layers.clone().expect("set-layer-opacity's diff pins a layers delta");
    assert_eq!(delta.patched[0].patch.opacity, Some(0.5), "the patch pins the opacity field");
}

/// 🔺️ The produced diff is EXACTLY the committed one: one `patched` entry setting `opacity`. The
/// committed `"fillJson": null` is what pins layer opacity as a lane of its own — a diff that
/// re-serialized the fill to fold the alpha in would be caught here and nowhere else.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-layer-opacity/dims-shape-a-to-half: produced diff differs from the committed 🔺️diff/🔣️.json");
    let delta = outcome.diff().layers.clone().expect("set-layer-opacity pins a layers delta");
    let patch = &delta.patched[0].patch;
    assert_eq!(patch.opacity, Some(0.5), "the opacity lane carries the new scalar");
    assert!(patch.fill_json.is_none(), "dimming a layer must not rewrite its fill to fold the alpha in");
    assert!(patch.visible.is_none(), "a half-opaque layer is not a hidden one");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `DrawingDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::drawing::DrawingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-layer-opacity/dims-shape-a-to-half: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::drawing::DrawingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::drawing::DrawingDiff as protocol::MutationDiff<DrawingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-layer-opacity/dims-shape-a-to-half: committed diff did not carry before to after");
}

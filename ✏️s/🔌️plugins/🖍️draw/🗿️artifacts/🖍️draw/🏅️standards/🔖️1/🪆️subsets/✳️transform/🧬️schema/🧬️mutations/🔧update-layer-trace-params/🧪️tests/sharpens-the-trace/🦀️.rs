//! 🧪️ `update-layer-trace-params` fixture — `sharpens-the-trace`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::draw::mutations::{apply_draw_mutation, inverse_draw_mutation, DrawMutation};
use crate::artifacts::draw::schema::find_draw_layer;
use crate::artifacts::draw::{DrawLayerNode, DrawSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> DrawSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> DrawSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> DrawMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_draw_mutation(&mut snapshot, &mutation()).expect("update-layer-trace-params applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "update-layer-trace-params/sharpens-the-trace: applied state differs from committed after-snapshot");
}

/// 🔧 `threshold` and `simplify_epsilon` are one cohesive facet: both move in the same step, and the
/// trace's `source_key` (the image it re-traces) is not part of that facet.
#[semio_framework_async_macros::async_test]
async fn both_trace_params_move_together_without_retargeting_the_source() {
    let base = before();
    let mut snapshot = base.clone();
    apply_draw_mutation(&mut snapshot, &mutation()).expect("update-layer-trace-params applies");
    let DrawLayerNode::Trace(before_trace) = find_draw_layer(&base, "trace-a").expect("before carries trace-a") else {
        panic!("sharpens-the-trace's before-snapshot must carry a trace layer");
    };
    let DrawLayerNode::Trace(after_trace) = find_draw_layer(&snapshot, "trace-a").expect("trace-a survives the params update") else {
        panic!("update-layer-trace-params must not change the layer's variant");
    };
    assert_eq!(before_trace.params.threshold, 0.5, "sharpens-the-trace's before-snapshot must start at the default threshold");
    assert_eq!(after_trace.params.threshold, 0.8, "the payload's threshold lands on the trace params");
    assert_eq!(after_trace.params.simplify_epsilon, 0.25, "the payload's simplify_epsilon lands in the SAME step");
    assert_eq!(after_trace.source_key, before_trace.source_key, "re-tuning the trace must not retarget its source image");
}

/// ↩️ The inverse is an `update-layer-trace-params` back to the params BASE carried.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_params() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_draw_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "a trace layer undoes with exactly one counter-update (a non-trace target would yield none)");
    let mut snapshot = base.clone();
    apply_draw_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_draw_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "update-layer-trace-params/sharpens-the-trace: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DrawSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-layer-trace-params/sharpens-the-trace: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-layer-trace-params/sharpens-the-trace: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches the diff builder: applied, with the params carried as one JSON
/// blob patch field.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "update-layer-trace-params/sharpens-the-trace declares an applied outcome");
    let produced = <DrawMutation as protocol::Mutation<DrawSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "update-layer-trace-params/sharpens-the-trace: the params really change, so no no-op warning is expected, got {:?}", produced.messages());
    let delta = produced.diff().layers.clone().expect("update-layer-trace-params's diff pins a layers delta");
    assert_eq!(delta.patched[0].id, "trace-a", "the patch is addressed to the trace layer");
    assert!(delta.patched[0].patch.trace_params_json.is_some(), "the params travel as one JSON-blob patch field");
}

/// 🔺️ The produced diff is EXACTLY the committed one: both trace parameters ride together in a single
/// `traceParamsJson` blob, and the trace's `sourceKey` appears nowhere in the diff — re-tuning a
/// trace can never retarget its source image.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <DrawMutation as protocol::Mutation<DrawSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "update-layer-trace-params/sharpens-the-trace: produced diff differs from the committed 🔺️diff/🔣️.json");
    let delta = outcome.diff().layers.clone().expect("update-layer-trace-params pins a layers delta");
    assert_eq!(delta.patched[0].id, "trace-a", "the entry addresses the trace layer");
    let patch = &delta.patched[0].patch;
    let blob = patch.trace_params_json.as_deref().expect("the trace-params lane is populated");
    let params: crate::artifacts::draw::DrawTraceParams = serde_json::from_str(blob).expect("the params blob is itself valid JSON");
    assert_eq!((params.threshold, params.simplify_epsilon), (0.8, 0.25), "both parameters ride in the one blob");
    assert!(patch.boolean_operation.is_none(), "the sibling variant-specific lane stays empty");
    assert!(!blob.contains("sourceKey"), "the trace source is not part of the params facet");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `DrawDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::draw::DrawDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-layer-trace-params/sharpens-the-trace: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::draw::DrawDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::draw::DrawDiff as protocol::MutationDiff<DrawSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "update-layer-trace-params/sharpens-the-trace: committed diff did not carry before to after");
}

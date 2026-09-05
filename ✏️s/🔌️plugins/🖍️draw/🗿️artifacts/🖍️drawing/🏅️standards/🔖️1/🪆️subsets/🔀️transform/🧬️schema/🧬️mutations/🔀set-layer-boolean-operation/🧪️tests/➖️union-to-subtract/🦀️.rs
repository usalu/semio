//! 🧪️ `set-layer-boolean-operation` fixture — `➖️union-to-subtract`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::drawing::mutations::{apply_drawing_mutation, inverse_drawing_mutation, DrawingMutation};
use crate::artifacts::drawing::schema::find_drawing_layer;
use crate::artifacts::drawing::{DrawingLayerNode, DrawingSnapshot};

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
    apply_drawing_mutation(&mut snapshot, &mutation()).expect("set-layer-boolean-operation applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "set-layer-boolean-operation/union-to-subtract: applied state differs from committed after-snapshot");
}

/// 🔀 `operation` lives on the Boolean VARIANT, not on the shared layer base — so the operand list
/// must ride through untouched, and a plain shape layer in the same document is unaffected.
#[semio_framework_async_macros::async_test]
async fn only_the_boolean_variants_operation_changes() {
    let base = before();
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation()).expect("set-layer-boolean-operation applies");
    let DrawingLayerNode::Boolean(before_boolean) = find_drawing_layer(&base, "boolean-a").expect("before carries boolean-a") else {
        panic!("union-to-subtract's before-snapshot must carry a boolean layer");
    };
    let DrawingLayerNode::Boolean(after_boolean) = find_drawing_layer(&snapshot, "boolean-a").expect("boolean-a survives the operation change") else {
        panic!("set-layer-boolean-operation must not change the layer's variant");
    };
    assert_eq!(before_boolean.operation, "union", "union-to-subtract's before-snapshot must start on union");
    assert_eq!(after_boolean.operation, "subtract", "set-layer-boolean-operation must write the payload's boolean_operation");
    assert_eq!(after_boolean.children, before_boolean.children, "changing the operation must not disturb the operand list");
    assert_eq!(find_drawing_layer(&snapshot, "shape-a"), find_drawing_layer(&base, "shape-a"), "the sibling shape layer is out of this mutation's reach");
}

/// ↩️ The inverse is a `set-layer-boolean-operation` back to the operation BASE carried.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_operation() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_drawing_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "a boolean layer undoes with exactly one counter-set (a non-boolean target would yield none)");
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_drawing_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "set-layer-boolean-operation/union-to-subtract: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-layer-boolean-operation/union-to-subtract: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-layer-boolean-operation/union-to-subtract: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches the diff builder. This verb's no-op guard only fires for a
/// Boolean layer already carrying the requested operation; `boolean-a` is on `union`, so the
/// outcome must be clean.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-layer-boolean-operation/union-to-subtract declares an applied outcome");
    let produced = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "set-layer-boolean-operation/union-to-subtract: subtract differs from union, so no no-op warning is expected, got {:?}", produced.messages());
    let delta = produced.diff().layers.clone().expect("set-layer-boolean-operation's diff pins a layers delta");
    assert_eq!(delta.patched[0].id, "boolean-a", "the patch is addressed to the boolean layer");
    assert_eq!(delta.patched[0].patch.boolean_operation.as_deref(), Some("subtract"), "the patch pins the boolean_operation field");
}

/// 🔺️ The produced diff is EXACTLY the committed one: one `patched` entry addressing `boolean-a` and
/// setting the variant-specific `booleanOperation` lane. The sibling shape layer never appears in the
/// diff at all — only the Boolean layer is touched.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-layer-boolean-operation/union-to-subtract: produced diff differs from the committed 🔺️diff/🔣️.json");
    let delta = outcome.diff().layers.clone().expect("set-layer-boolean-operation pins a layers delta");
    assert_eq!(delta.patched.len(), 1, "exactly one layer is patched");
    assert_eq!(delta.patched[0].id, "boolean-a", "the entry addresses the boolean layer, never its operands");
    let patch = &delta.patched[0].patch;
    assert_eq!(patch.boolean_operation.as_deref(), Some("subtract"), "the boolean-operation lane carries the new operation");
    assert!(patch.trace_params_json.is_none(), "the sibling variant-specific lane stays empty");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `DrawingDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::drawing::DrawingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-layer-boolean-operation/union-to-subtract: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::drawing::DrawingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::drawing::DrawingDiff as protocol::MutationDiff<DrawingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-layer-boolean-operation/union-to-subtract: committed diff did not carry before to after");
}

//! 🧪️ `update-layer-transform` fixture — `translates-and-scales-shape-a`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::draw::mutations::{apply_draw_mutation, inverse_draw_mutation, DrawMutation};
use crate::artifacts::draw::schema::{find_draw_layer, layer_base};
use crate::artifacts::draw::{DrawLayerNode, DrawSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

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
    apply_draw_mutation(&mut snapshot, &mutation()).expect("update-layer-transform applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "update-layer-transform/translates-and-scales-shape-a: applied state differs from committed after-snapshot");
}

/// 🔄️ The `update` verb's cohesive-facet case: position, scale and rotation move as ONE atomic
/// value, and the shape's own local geometry (`rect`) is untouched by the layer transform.
#[semio_framework_async_macros::async_test]
async fn the_whole_transform_facet_moves_atomically() {
    let base = before();
    let mut snapshot = base.clone();
    apply_draw_mutation(&mut snapshot, &mutation()).expect("update-layer-transform applies");
    let after_layer = layer_base(find_draw_layer(&snapshot, "shape-a").expect("shape-a survives a transform update"));
    assert_eq!(after_layer.transform.x, 24.0, "the payload's x lands on the layer transform");
    assert_eq!(after_layer.transform.y, -8.0, "the payload's y lands on the layer transform");
    assert_eq!(after_layer.transform.scale_x, 2.0, "the payload's scale_x lands on the layer transform");
    assert_eq!(after_layer.transform.scale_y, 1.5, "the payload's scale_y lands on the layer transform");
    assert_eq!(after_layer.transform.rotation, 0.0, "the payload's rotation lands on the layer transform");
    let (DrawLayerNode::Shape(before_shape), DrawLayerNode::Shape(after_shape)) = (find_draw_layer(&base, "shape-a").expect("before carries shape-a"), find_draw_layer(&snapshot, "shape-a").expect("after carries shape-a")) else {
        panic!("translates-and-scales-shape-a targets a shape layer");
    };
    assert_eq!(after_shape.rect, before_shape.rect, "a layer transform never bakes itself into the shape's local geometry");
}

/// ↩️ The inverse is an `update-layer-transform` back to the transform BASE carried.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_transform() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_draw_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "update-layer-transform undoes with exactly one counter-update");
    let mut snapshot = base.clone();
    apply_draw_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_draw_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "update-layer-transform/translates-and-scales-shape-a: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DrawSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-layer-transform/translates-and-scales-shape-a: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-layer-transform/translates-and-scales-shape-a: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches the diff builder. Two `mutation.invariant` Fatals guard this
/// verb — non-finite components and non-positive scale — and both must stay silent for this payload.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "update-layer-transform/translates-and-scales-shape-a declares an applied outcome");
    let produced = <DrawMutation as protocol::Mutation<DrawSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "update-layer-transform/translates-and-scales-shape-a: every component is finite and both scales are positive, so no invariant may fire, got {:?}", produced.messages());
    let delta = produced.diff().layers.clone().expect("update-layer-transform's diff pins a layers delta");
    assert!(delta.patched[0].patch.transform_json.is_some(), "the transform travels as one JSON-blob patch field, not five scalars");
}

/// 🔺️ The produced diff is EXACTLY the committed one. The whole transform facet travels as ONE
/// `transformJson` blob string — five scalars in one lane, matching the `update` verb's
/// cohesive-facet contract — rather than five independent patch fields that could land apart.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <DrawMutation as protocol::Mutation<DrawSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "update-layer-transform/translates-and-scales-shape-a: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let delta = outcome.diff().layers.clone().expect("update-layer-transform pins a layers delta");
    let patch = &delta.patched[0].patch;
    let blob = patch.transform_json.as_deref().expect("the transform lane is populated");
    let transform: crate::artifacts::draw::DrawTransform = serde_json::from_str(blob).expect("the transform blob is itself valid JSON");
    assert_eq!((transform.x, transform.y, transform.scale_x, transform.scale_y, transform.rotation), (24.0, -8.0, 2.0, 1.5, 0.0), "all five components ride in the single blob");
    assert!(patch.fill_json.is_none() && patch.stroke_json.is_none() && patch.trace_params_json.is_none(), "no other blob lane is written");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `DrawDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::draw::DrawDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-layer-transform/translates-and-scales-shape-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::draw::DrawDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::draw::DrawDiff as protocol::MutationDiff<DrawSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "update-layer-transform/translates-and-scales-shape-a: committed diff did not carry before to after");
}

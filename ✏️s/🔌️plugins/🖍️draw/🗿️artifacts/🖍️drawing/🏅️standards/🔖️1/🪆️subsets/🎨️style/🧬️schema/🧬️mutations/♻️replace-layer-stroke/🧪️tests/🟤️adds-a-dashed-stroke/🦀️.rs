//! 🧪️ `replace-layer-stroke` fixture — `🟤️adds-a-dashed-stroke`.
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
    apply_drawing_mutation(&mut snapshot, &mutation()).expect("replace-layer-stroke applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-layer-stroke/adds-a-dashed-stroke: applied state differs from committed after-snapshot");
}

/// ♻️ `stroke` is an OPTION: this case goes `None → Some(..)`, and the optional `dash` array inside
/// the new stroke must survive verbatim rather than being normalized away.
#[semio_framework_async_macros::async_test]
async fn an_absent_stroke_becomes_the_committed_dashed_one() {
    let base = before();
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation()).expect("replace-layer-stroke applies");
    let before_attributes = layer_base(find_drawing_layer(&base, "shape-a").expect("before carries shape-a")).attributes.clone();
    let after_attributes = layer_base(find_drawing_layer(&snapshot, "shape-a").expect("shape-a survives a stroke swap")).attributes.clone();
    assert_eq!(before_attributes.stroke, None, "adds-a-dashed-stroke's before-snapshot must start with no stroke at all");
    let stroke = after_attributes.stroke.expect("replace-layer-stroke must install the payload's stroke");
    assert_eq!(stroke.width, 2.0, "the stroke width comes from the payload");
    assert_eq!(stroke.cap, "round", "the stroke cap comes from the payload");
    assert_eq!(stroke.join, "bevel", "the stroke join comes from the payload");
    assert_eq!(stroke.dash, Some(vec![4.0, 2.0]), "the optional dash pattern must survive verbatim");
    assert_eq!(after_attributes.fill, before_attributes.fill, "installing a stroke must not disturb the fill");
}

/// ↩️ The inverse is a `replace-layer-stroke` back to BASE's own `None`, dropping the stroke again.
#[semio_framework_async_macros::async_test]
async fn inverse_drops_the_stroke_again() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_drawing_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "replace-layer-stroke undoes with exactly one counter-replace");
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_drawing_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-layer-stroke/adds-a-dashed-stroke: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-layer-stroke/adds-a-dashed-stroke: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-layer-stroke/adds-a-dashed-stroke: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches the diff builder: applied, and only the stroke patch field is
/// populated.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-layer-stroke/adds-a-dashed-stroke declares an applied outcome");
    let produced = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "replace-layer-stroke/adds-a-dashed-stroke: None differs from Some(..), so no no-op warning is expected, got {:?}", produced.messages());
    let delta = produced.diff().layers.clone().expect("replace-layer-stroke's diff pins a layers delta");
    assert!(delta.patched[0].patch.stroke_json.is_some(), "the stroke patch field must be populated");
    assert_eq!(delta.patched[0].patch.fill_json, None, "a stroke replace must leave the fill patch field empty");
}

/// 🔺️ The produced diff is EXACTLY the committed one. `strokeJson` holds a serialized `Option`, so an
/// absent stroke would be the literal string `"null"` rather than an absent lane — this case is the
/// `Some(..)` side, and the optional `dash` array survives inside the blob.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-layer-stroke/adds-a-dashed-stroke: produced diff differs from the committed 🔺️diff/🔣️.json");
    let delta = outcome.diff().layers.clone().expect("replace-layer-stroke pins a layers delta");
    let patch = &delta.patched[0].patch;
    let blob = patch.stroke_json.as_deref().expect("the stroke lane is populated");
    let stroke: Option<crate::artifacts::drawing::StrokeStyle> = serde_json::from_str(blob).expect("the stroke blob is itself valid JSON");
    let stroke = stroke.expect("this case installs a stroke rather than clearing one");
    assert_eq!(stroke.dash, Some(vec![4.0, 2.0]), "the optional dash pattern survives inside the blob");
    assert!(patch.fill_json.is_none(), "a stroke swap must leave the fill lane empty");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `DrawingDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::drawing::DrawingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-layer-stroke/adds-a-dashed-stroke: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::drawing::DrawingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::drawing::DrawingDiff as protocol::MutationDiff<DrawingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-layer-stroke/adds-a-dashed-stroke: committed diff did not carry before to after");
}

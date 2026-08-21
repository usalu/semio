//! 🧪️ `replace-layer-fill` fixture — `solid-to-linear-gradient`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::draw::mutations::{apply_draw_mutation, inverse_draw_mutation, DrawMutation};
use crate::artifacts::draw::schema::{find_draw_layer, layer_base};
use crate::artifacts::draw::{DrawSnapshot, FillStyle};

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
    apply_draw_mutation(&mut snapshot, &mutation()).expect("replace-layer-fill applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-layer-fill/solid-to-linear-gradient: applied state differs from committed after-snapshot");
}

/// 🔁 `fill` is a TAGGED union, so `replace` swaps the whole value — including its variant. The
/// solid's `color` must not survive as a leftover field of the gradient.
#[semio_framework_async_macros::async_test]
async fn the_fill_variant_itself_is_swapped() {
    let base = before();
    let mut snapshot = base.clone();
    apply_draw_mutation(&mut snapshot, &mutation()).expect("replace-layer-fill applies");
    let before_fill = layer_base(find_draw_layer(&base, "shape-a").expect("before carries shape-a")).attributes.fill.clone();
    let after_fill = layer_base(find_draw_layer(&snapshot, "shape-a").expect("shape-a survives a fill swap")).attributes.fill.clone();
    assert!(matches!(before_fill, Some(FillStyle::Solid { .. })), "solid-to-linear-gradient's before-snapshot must start on a solid fill");
    let Some(FillStyle::LinearGradient { x2, stops, .. }) = after_fill else {
        panic!("replace-layer-fill must land on the payload's linear-gradient variant");
    };
    assert_eq!(x2, 120.0, "the gradient's own geometry comes from the payload, not from the old solid");
    assert_eq!(stops.len(), 2, "both committed gradient stops must survive the swap");
    let stroke = layer_base(find_draw_layer(&snapshot, "shape-a").expect("shape-a is present")).attributes.stroke.clone();
    assert_eq!(stroke, None, "replacing the fill must not invent or drop a stroke");
}

/// ↩️ The inverse is a `replace-layer-fill` back to the fill payload BASE carried.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_fill() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_draw_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "replace-layer-fill undoes with exactly one counter-replace");
    let mut snapshot = base.clone();
    apply_draw_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_draw_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-layer-fill/solid-to-linear-gradient: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DrawSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-layer-fill/solid-to-linear-gradient: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-layer-fill/solid-to-linear-gradient: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches the diff builder: applied, and the fill travels as a JSON blob
/// patch field so a tagged union survives every representation intact.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-layer-fill/solid-to-linear-gradient declares an applied outcome");
    let produced = <DrawMutation as protocol::Mutation<DrawSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "replace-layer-fill/solid-to-linear-gradient: the fill really changes, so no no-op warning is expected, got {:?}", produced.messages());
    let delta = produced.diff().layers.clone().expect("replace-layer-fill's diff pins a layers delta");
    assert!(delta.patched[0].patch.fill_json.is_some(), "the fill patch field must be populated");
    assert_eq!(delta.patched[0].patch.stroke_json, None, "a fill replace must leave the stroke patch field empty");
}

/// 🔺️ The produced diff is EXACTLY the committed one. The fill rides as a `fillJson` blob holding the
/// TAGGED union — `"kind":"linearGradient"` and its stops — so the variant switch survives every
/// representation. The committed `"strokeJson": null` pins that a fill swap leaves the stroke alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <DrawMutation as protocol::Mutation<DrawSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-layer-fill/solid-to-linear-gradient: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let delta = outcome.diff().layers.clone().expect("replace-layer-fill pins a layers delta");
    let patch = &delta.patched[0].patch;
    let blob = patch.fill_json.as_deref().expect("the fill lane is populated");
    let fill: Option<crate::artifacts::draw::FillStyle> = serde_json::from_str(blob).expect("the fill blob is itself valid JSON");
    assert!(matches!(fill, Some(crate::artifacts::draw::FillStyle::LinearGradient { .. })), "the blob carries the tagged gradient variant, not a bare colour");
    assert!(patch.stroke_json.is_none(), "a fill swap must leave the stroke lane empty");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `DrawDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::draw::DrawDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-layer-fill/solid-to-linear-gradient: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::draw::DrawDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::draw::DrawDiff as protocol::MutationDiff<DrawSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-layer-fill/solid-to-linear-gradient: committed diff did not carry before to after");
}

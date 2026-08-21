//! 🧪️ `set-layer-visible` fixture — `hides-shape-a`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::draw::mutations::{apply_draw_mutation, inverse_draw_mutation, DrawMutation};
use crate::artifacts::draw::schema::{find_draw_layer, layer_base};
use crate::artifacts::draw::DrawSnapshot;

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
    apply_draw_mutation(&mut snapshot, &mutation()).expect("set-layer-visible applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "set-layer-visible/hides-shape-a: applied state differs from committed after-snapshot");
}

/// 👁️ `visible` is the ONLY base field this setter writes — `locked`, `opacity` and `blend_mode`
/// each have their own mutation and must survive a visibility flip untouched.
#[semio_framework_async_macros::async_test]
async fn only_the_visible_flag_flips() {
    let base = before();
    let mut snapshot = base.clone();
    apply_draw_mutation(&mut snapshot, &mutation()).expect("set-layer-visible applies");
    let before_layer = layer_base(find_draw_layer(&base, "shape-a").expect("before carries shape-a"));
    let after_layer = layer_base(find_draw_layer(&snapshot, "shape-a").expect("shape-a survives a visibility flip"));
    assert!(before_layer.visible, "hides-shape-a's before-snapshot must start visible");
    assert!(!after_layer.visible, "set-layer-visible must write the payload's visible flag");
    assert_eq!(after_layer.locked, before_layer.locked, "set-layer-visible must not touch locked");
    assert_eq!(after_layer.opacity, before_layer.opacity, "hiding a layer is not the same as zeroing its opacity");
    assert_eq!(after_layer.blend_mode, before_layer.blend_mode, "set-layer-visible must not touch blend_mode");
}

/// ↩️ The inverse is a `set-layer-visible` back to the flag BASE carried.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_visibility() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_draw_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "set-layer-visible undoes with exactly one counter-set");
    let mut snapshot = base.clone();
    apply_draw_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_draw_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "set-layer-visible/hides-shape-a: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DrawSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-layer-visible/hides-shape-a: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-layer-visible/hides-shape-a: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches the diff builder: applied, no `mutation.no-op` warning, and the
/// sparse delta is a single `visible` patch on `shape-a` — never a whole-layer replacement.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-layer-visible/hides-shape-a declares an applied outcome");
    let produced = <DrawMutation as protocol::Mutation<DrawSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "set-layer-visible/hides-shape-a: the flag really flips, so no no-op warning is expected, got {:?}", produced.messages());
    let delta = produced.diff().layers.clone().expect("set-layer-visible's diff pins a layers delta");
    assert_eq!(delta.patched.len(), 1, "set-layer-visible patches exactly one layer");
    assert_eq!(delta.patched[0].id, "shape-a", "the patch is addressed to the payload's layer_id");
    assert_eq!(delta.patched[0].patch.visible, Some(false), "the patch pins the visible field");
    assert_eq!(delta.patched[0].patch.layer_json, None, "a visibility flip must never fall back to a whole-layer JSON replacement");
    assert!(delta.added.is_empty() && delta.removed.is_empty(), "set-layer-visible is structurally inert");
}

/// 🔺️ The produced diff is EXACTLY the committed one: a single `patched` entry whose `DrawLayerPatch`
/// sets `visible` and leaves its other ten fields `null`. `DrawLayerPatch` has no
/// `skip_serializing_if`, so those nulls are committed explicitly — that is what proves a hide never
/// degrades into a whole-layer `layerJson` replacement.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <DrawMutation as protocol::Mutation<DrawSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-layer-visible/hides-shape-a: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let delta = outcome.diff().layers.clone().expect("set-layer-visible pins a layers delta");
    let patch = &delta.patched[0].patch;
    assert_eq!(patch.visible, Some(false), "the visible lane carries the new flag");
    assert!(patch.locked.is_none() && patch.opacity.is_none() && patch.blend_mode.is_none(), "the sibling base-field lanes stay empty");
    assert!(patch.layer_json.is_none(), "a visibility flip must never fall back to a whole-layer replacement");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `DrawDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::draw::DrawDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-layer-visible/hides-shape-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::draw::DrawDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::draw::DrawDiff as protocol::MutationDiff<DrawSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-layer-visible/hides-shape-a: committed diff did not carry before to after");
}

//! 🧪️ `reorder-layer` fixture — `moves-shape-a-above-shape-b`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::draw::mutations::{apply_draw_mutation, inverse_draw_mutation, DrawMutation};
use crate::artifacts::draw::schema::{find_draw_layer, layer_id};
use crate::artifacts::draw::DrawSnapshot;

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
    apply_draw_mutation(&mut snapshot, &mutation()).expect("reorder-layer applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "reorder-layer/moves-shape-a-above-shape-b: applied state differs from committed after-snapshot");
}

/// 🔃 `reorder-layer` is a stacking-order move, never a spatial one: the ids swap positions in the
/// root list while both layers keep their own transforms and geometry untouched.
#[semio_framework_async_macros::async_test]
async fn stacking_order_changes_but_geometry_does_not() {
    let base = before();
    assert_eq!(layer_id(&base.layers[0]), "shape-a", "moves-shape-a-above-shape-b's before-snapshot must start with shape-a first");
    let mut snapshot = base.clone();
    apply_draw_mutation(&mut snapshot, &mutation()).expect("reorder-layer applies");
    assert_eq!(snapshot.layers.len(), base.layers.len(), "a reorder never changes the member count");
    assert_eq!(layer_id(&snapshot.layers[0]), "shape-b", "the untouched sibling slides down to index 0");
    assert_eq!(layer_id(&snapshot.layers[1]), "shape-a", "the addressed layer lands at the payload's index");
    assert_eq!(find_draw_layer(&snapshot, "shape-a"), find_draw_layer(&base, "shape-a"), "the moved layer's own value is carried across untouched");
}

/// ↩️ The inverse is a `reorder-layer` back to the exact `(parent_id, index)` address BASE showed.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_address() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_draw_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "reorder-layer undoes with exactly one counter-reorder");
    let mut snapshot = base.clone();
    apply_draw_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_draw_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "reorder-layer/moves-shape-a-above-shape-b: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DrawSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "reorder-layer/moves-shape-a-above-shape-b: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "reorder-layer/moves-shape-a-above-shape-b: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches the diff builder: applied (the no-op guard only fires when the
/// requested `(parent, index)` already equals BASE's), expressed as a remove-then-insert pair rather
/// than a whole-list permutation.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "reorder-layer/moves-shape-a-above-shape-b declares an applied outcome");
    let produced = <DrawMutation as protocol::Mutation<DrawSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "reorder-layer/moves-shape-a-above-shape-b: index 1 differs from BASE's index 0, so no no-op warning is expected, got {:?}", produced.messages());
    let delta = produced.diff().layers.clone().expect("reorder-layer's diff pins a layers delta");
    assert_eq!(delta.removed, vec!["shape-a".to_string()], "the move lifts the addressed layer out first");
    assert_eq!(delta.added.len(), 1, "and re-inserts exactly that one layer");
    assert_eq!(delta.added[0].index, 1, "at the payload's FINAL-state index");
    assert_eq!(delta.reordered, None, "a single-layer move must not degrade into a whole-root permutation");
}

/// 🔺️ The produced diff is EXACTLY the committed one: a remove-then-insert PAIR carrying the moved
/// layer's own value to its new index. The committed `"reordered": null` is the load-bearing part —
/// `DrawLayersDelta` also offers a whole-root permutation lane, and a single-layer move must not
/// reach for it, nor may the untouched sibling appear anywhere in the diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <DrawMutation as protocol::Mutation<DrawSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "reorder-layer/moves-shape-a-above-shape-b: produced diff differs from the committed 🔺️diff/🔣️.json");
    let delta = outcome.diff().layers.clone().expect("reorder-layer pins a layers delta");
    assert_eq!(delta.removed, vec!["shape-a".to_string()], "the move lifts the addressed layer out first");
    assert_eq!(delta.added.len(), 1, "and re-inserts exactly that one layer");
    assert_eq!(delta.added[0].index, 1, "at the payload's FINAL-state index");
    assert_eq!(delta.reordered, None, "a single-layer move must not degrade into a whole-root permutation");
    assert!(!DIFF.contains("shape-b"), "the untouched sibling must not appear in the committed diff");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `DrawDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::draw::DrawDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "reorder-layer/moves-shape-a-above-shape-b: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::draw::DrawDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::draw::DrawDiff as protocol::MutationDiff<DrawSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-layer/moves-shape-a-above-shape-b: committed diff did not carry before to after");
}

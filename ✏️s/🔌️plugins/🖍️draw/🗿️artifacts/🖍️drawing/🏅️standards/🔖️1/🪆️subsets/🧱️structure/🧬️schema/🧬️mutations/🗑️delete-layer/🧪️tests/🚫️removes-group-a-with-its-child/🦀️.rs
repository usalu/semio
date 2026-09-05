//! 🧪️ `delete-layer` fixture — `🚫️removes-group-a-with-its-child`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::drawing::mutations::{apply_drawing_mutation, inverse_drawing_mutation, DrawingMutation};
use crate::artifacts::drawing::schema::find_drawing_layer;
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
    apply_drawing_mutation(&mut snapshot, &mutation()).expect("delete-layer applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-layer/removes-group-a-with-its-child: applied state differs from committed after-snapshot");
}

/// 🗑️ Deleting a GROUP takes its whole subtree with it — the nested `text-a` must become
/// unaddressable too, not be reparented to the root.
#[semio_framework_async_macros::async_test]
async fn deleting_a_group_takes_its_subtree() {
    let base = before();
    assert!(find_drawing_layer(&base, "text-a").is_some(), "removes-group-a-with-its-child's before-snapshot must nest text-a inside group-a");
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation()).expect("delete-layer applies");
    assert!(find_drawing_layer(&snapshot, "group-a").is_none(), "the addressed group must be gone");
    assert!(find_drawing_layer(&snapshot, "text-a").is_none(), "the group's child goes with it — a delete never reparents a subtree to the root");
    assert_eq!(snapshot.layers.len(), 1, "only the untouched sibling remains at the root");
    assert!(find_drawing_layer(&snapshot, "shape-a").is_some(), "the sibling layer survives the delete");
}

/// ↩️ The inverse is a `create-layer` carrying the FULL removed subtree back to its exact captured
/// (parent, index) address.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_whole_subtree_at_its_old_address() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_drawing_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "delete-layer undoes with exactly one create-layer, subtree included");
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_drawing_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-layer/removes-group-a-with-its-child: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-layer/removes-group-a-with-its-child: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-layer/removes-group-a-with-its-child: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches the diff builder: applied, with a `removed`-only delta naming
/// the group alone — the child is implied by the tree, never listed separately.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-layer/removes-group-a-with-its-child declares an applied outcome");
    let produced = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "delete-layer/removes-group-a-with-its-child: group-a exists, so target-missing must not fire, got {:?}", produced.messages());
    let delta = produced.diff().layers.clone().expect("delete-layer's diff pins a layers delta");
    assert_eq!(delta.removed, vec!["group-a".to_string()], "the delta names only the addressed group");
    assert!(delta.added.is_empty() && delta.patched.is_empty(), "delete-layer is a pure removal");
}

/// 🔺️ The produced diff is EXACTLY the committed one: a `removed` list naming ONLY the group. The
/// nested `text-a` is implied by the tree and is deliberately absent from the diff — a delete that
/// enumerated its descendants would be describing the outcome instead of the change.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-layer/removes-group-a-with-its-child: produced diff differs from the committed 🔺️diff/🔣️.json");
    let delta = outcome.diff().layers.clone().expect("delete-layer pins a layers delta");
    assert_eq!(delta.removed, vec!["group-a".to_string()], "only the addressed group is named");
    assert!(delta.added.is_empty() && delta.patched.is_empty(), "a delete is neither a move nor a patch");
    assert!(!DIFF.contains("text-a"), "the nested child must not be enumerated in the committed diff");
    assert!(!DIFF.contains("shape-a"), "the untouched sibling must not appear in the committed diff");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `DrawingDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::drawing::DrawingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-layer/removes-group-a-with-its-child: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::drawing::DrawingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::drawing::DrawingDiff as protocol::MutationDiff<DrawingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-layer/removes-group-a-with-its-child: committed diff did not carry before to after");
}

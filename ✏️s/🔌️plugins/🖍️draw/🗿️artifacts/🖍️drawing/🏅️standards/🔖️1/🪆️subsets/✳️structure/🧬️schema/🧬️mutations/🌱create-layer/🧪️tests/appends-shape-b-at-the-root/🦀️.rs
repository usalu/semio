//! 🧪️ `create-layer` fixture — `appends-shape-b-at-the-root`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::drawing::mutations::{apply_drawing_mutation, inverse_drawing_mutation, DrawingMutation};
use crate::artifacts::drawing::schema::{find_drawing_layer, layer_id};
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
    apply_drawing_mutation(&mut snapshot, &mutation()).expect("create-layer applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "create-layer/appends-shape-b-at-the-root: applied state differs from committed after-snapshot");
}

/// 🌱 Both `parent_id` and `index` are omitted from this payload, so the diff builder must resolve
/// the address itself: root container, append position — i.e. BASE's own root length.
#[semio_framework_async_macros::async_test]
async fn an_unaddressed_create_appends_at_the_root_end() {
    let base = before();
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation()).expect("create-layer applies");
    assert_eq!(snapshot.layers.len(), base.layers.len() + 1, "create-layer adds exactly one root layer");
    assert_eq!(layer_id(snapshot.layers.last().expect("the root list is non-empty")), "shape-b", "an omitted index appends at the END of the root list");
    assert_eq!(layer_id(&snapshot.layers[0]), "shape-a", "the pre-existing root layer keeps its position");
    assert!(find_drawing_layer(&base, "shape-b").is_none(), "appends-shape-b-at-the-root's before-snapshot must not already carry shape-b");
}

/// ↩️ The inverse is a `delete-layer` of the id the payload itself carried — `create-layer` needs no
/// BASE lookup to know what to undo.
#[semio_framework_async_macros::async_test]
async fn inverse_deletes_the_layer_it_created() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_drawing_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "create-layer undoes with exactly one delete-layer");
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_drawing_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "create-layer/appends-shape-b-at-the-root: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-layer/appends-shape-b-at-the-root: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-layer/appends-shape-b-at-the-root: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches the diff builder. `create-layer` guards two Fatals —
/// `mutation.duplicate-id` for a colliding id and `mutation.invariant` for a parent that is not a
/// group — and neither may fire for an unaddressed create of a fresh id.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-layer/appends-shape-b-at-the-root declares an applied outcome");
    let produced = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "create-layer/appends-shape-b-at-the-root: shape-b is a fresh id and no parent is named, so neither guard may fire, got {:?}", produced.messages());
    let delta = produced.diff().layers.clone().expect("create-layer's diff pins a layers delta");
    assert_eq!(delta.added.len(), 1, "create-layer adds exactly one layer");
    assert_eq!(delta.added[0].parent_id, None, "an omitted parent_id stays None — a root insert");
    assert_eq!(delta.added[0].index, 1, "the resolved append index is BASE's own root length");
    assert!(delta.removed.is_empty() && delta.patched.is_empty(), "create-layer is a pure insert");
}

/// 🔺️ The produced diff is EXACTLY the committed one: a single `added` entry carrying the whole new
/// layer plus its resolved ADDRESS. `DrawingLayerAddition.parent_id` is the one field in this diff
/// family with `skip_serializing_if`, so a root insert omits the key entirely rather than writing
/// `null` — and the pre-existing `shape-a` appears nowhere, because an insert is not a rewrite.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-layer/appends-shape-b-at-the-root: produced diff differs from the committed 🔺️diff/🔣️.json");
    let delta = outcome.diff().layers.clone().expect("create-layer pins a layers delta");
    assert_eq!(delta.added.len(), 1, "exactly one layer is inserted");
    assert_eq!(delta.added[0].parent_id, None, "an omitted parent stays a root insert");
    assert_eq!(delta.added[0].index, 1, "the append index was resolved from BASE's own root length");
    assert!(delta.removed.is_empty() && delta.patched.is_empty(), "a create touches neither existing layer");
    assert!(!DIFF.contains("shape-a"), "the untouched sibling must not appear anywhere in the committed diff");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `DrawingDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::drawing::DrawingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-layer/appends-shape-b-at-the-root: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::drawing::DrawingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::drawing::DrawingDiff as protocol::MutationDiff<DrawingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-layer/appends-shape-b-at-the-root: committed diff did not carry before to after");
}

//! 🧪️ `rename-layer` fixture — `renames-shape-a-without-touching-its-id`.
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
    apply_drawing_mutation(&mut snapshot, &mutation()).expect("rename-layer applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "rename-layer/renames-shape-a-without-touching-its-id: applied state differs from committed after-snapshot");
}

/// ✏️ In drawing, a layer's `name` is its human label and `id` is its identity key — unlike dag, where
/// `rename-node` rewrites the id itself. `rename-layer` must leave the id (and therefore every
/// reference to it) alone.
#[semio_framework_async_macros::async_test]
async fn the_layer_identity_survives_the_rename() {
    let base = before();
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation()).expect("rename-layer applies");
    assert!(find_drawing_layer(&snapshot, "shape-a").is_some(), "rename-layer must not change the layer's addressable id");
    let before_layer = layer_base(find_drawing_layer(&base, "shape-a").expect("before carries shape-a"));
    let after_layer = layer_base(find_drawing_layer(&snapshot, "shape-a").expect("shape-a is still addressable by id"));
    assert_eq!(before_layer.name, "Alpha", "the before-snapshot must start with the original label");
    assert_eq!(after_layer.name, "Alpha Renamed", "rename-layer must write the payload's new_name");
    assert_eq!(after_layer.id, before_layer.id, "the identity key is untouched by a rename");
}

/// ↩️ The inverse is a `rename-layer` back to the label BASE carried.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_name() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_drawing_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "rename-layer undoes with exactly one counter-rename");
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_drawing_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "rename-layer/renames-shape-a-without-touching-its-id: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rename-layer/renames-shape-a-without-touching-its-id: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "rename-layer/renames-shape-a-without-touching-its-id: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches the diff builder: applied, and the delta pins `name`.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-layer/renames-shape-a-without-touching-its-id declares an applied outcome");
    let produced = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "rename-layer/renames-shape-a-without-touching-its-id: the label really changes, so no no-op warning is expected, got {:?}", produced.messages());
    let delta = produced.diff().layers.clone().expect("rename-layer's diff pins a layers delta");
    assert_eq!(delta.patched[0].patch.name.as_deref(), Some("Alpha Renamed"), "the patch pins the name field");
    assert!(delta.removed.is_empty(), "a rename never removes and re-adds the layer");
}

/// 🔺️ The produced diff is EXACTLY the committed one: one `patched` entry setting `name`. The entry's
/// own `id` addresses `shape-a` and is NOT part of the patch — drawing's identity key is never a patch
/// field, which is exactly why a rename here cannot become a re-identification.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rename-layer/renames-shape-a-without-touching-its-id: produced diff differs from the committed 🔺️diff/🔣️.json");
    let delta = outcome.diff().layers.clone().expect("rename-layer pins a layers delta");
    assert_eq!(delta.patched[0].id, "shape-a", "the entry addresses the layer by its unchanged identity key");
    let patch = &delta.patched[0].patch;
    assert_eq!(patch.name.as_deref(), Some("Alpha Renamed"), "the name lane carries the new label");
    assert!(patch.layer_json.is_none(), "a rename must not degrade into a whole-layer replacement");
    assert!(delta.removed.is_empty() && delta.added.is_empty(), "a rename is never expressed as a remove-and-re-add");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `DrawingDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::drawing::DrawingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "rename-layer/renames-shape-a-without-touching-its-id: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::drawing::DrawingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::drawing::DrawingDiff as protocol::MutationDiff<DrawingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-layer/renames-shape-a-without-touching-its-id: committed diff did not carry before to after");
}

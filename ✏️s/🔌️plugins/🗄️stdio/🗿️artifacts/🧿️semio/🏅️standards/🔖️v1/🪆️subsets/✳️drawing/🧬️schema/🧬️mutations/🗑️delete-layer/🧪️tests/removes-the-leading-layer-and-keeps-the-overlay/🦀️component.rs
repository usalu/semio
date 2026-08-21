//! 🧪️ `delete-layer` fixture — `removes-the-leading-layer-and-keeps-the-overlay`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: the layer is looked up BY ID but the diff
//! records its POSITION (`layers.removed[index]`) — an id-addressed mutation lowering to an
//! index-keyed diff, which is exactly the asymmetry this case pins. An unknown id is Error
//! `mutation.target-missing`.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::SemioDrawingDiff;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioDrawingSnapshot {
    serde_json::from_str(BEFORE).expect("delete-layer before snapshot decodes")
}
fn expected_after() -> SemioDrawingSnapshot {
    serde_json::from_str(AFTER).expect("delete-layer after snapshot decodes")
}
fn mutation() -> SemioDrawingMutation {
    serde_json::from_str(MUTATION).expect("delete-layer mutation decodes")
}

/// ▶️ The base layer goes; the overlay slides down to z-order 0.
#[semio_framework_async_macros::async_test]
async fn removes_the_layer_at_z_order_zero() {
    let base = before();
    assert_eq!(base.layers.len(), 2, "the fixture needs a second layer for the z-order shift to be observable");
    let produced = mutation().diff(&base).diff().apply(&base).expect("delete-layer applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "delete-layer/removes-the-leading-layer-and-keeps-the-overlay: applied state differs from the committed after-snapshot");
    assert!(!produced.layers.iter().any(|layer| layer.id == "l1"), "the named layer must be gone");
    assert_eq!(produced.layers, vec![base.layers[1].clone()], "the overlay slides down into z-order 0");
    assert_eq!(produced.styles, base.styles, "deleting a layer must not touch the style table its nodes referenced");
}

/// ↩️ The undo re-creates the layer AT ITS ORIGINAL INDEX with its whole node tree.
#[semio_framework_async_macros::async_test]
async fn the_undo_create_layer_restores_the_layer_at_its_original_index() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "delete-layer of an existing layer undoes as exactly one create-layer");
    let SemioDrawingMutation::CreateLayer(recreate) = &undo[0] else { panic!("delete-layer must undo as create-layer") };
    assert_eq!(recreate.index, 0, "the undo must re-insert at the ORIGINAL z-order, not append");
    assert_eq!(recreate.layer, base.layers[0], "and must recapture the whole layer, node tree included");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward delete-layer applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo create-layer applies");
    }
    assert_eq!(current, base, "delete-layer/removes-the-leading-layer-and-keeps-the-overlay: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"DeleteLayer":{"id":"l1"}}` payload are canonical — the payload names an ID while the diff records an INDEX.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioDrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-layer/removes-the-leading-layer-and-keeps-the-overlay: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("delete-layer mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("delete-layer mutation reparses");
    assert_eq!(reencoded, original, "delete-layer/removes-the-leading-layer-and-keeps-the-overlay: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the layer exists, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-layer/removes-the-leading-layer-and-keeps-the-overlay: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "deleting an existing layer must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `layers.removed`, carrying the POSITION rather than the id.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-layer/removes-the-leading-layer-and-keeps-the-overlay: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and the `canvas` slot — which NO leaf in
/// this subset ever writes — stays absent from it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed delete-layer diff decodes");
    assert!(decoded.canvas.is_none(), "no drawing mutation writes the canvas slot");
    let layers = decoded.layers.as_ref().expect("delete-layer must write the layers triple");
    assert_eq!(layers.removed, vec![0usize], "an index-keyed collection records the removal by position");
    assert!(layers.modified.is_empty() && layers.added.is_empty(), "a removal neither modifies nor adds");
    assert!(decoded.styles.is_none(), "the style table must stay untouched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-layer/removes-the-leading-layer-and-keeps-the-overlay: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed delete-layer diff decodes");
    let produced = decoded.apply(&before()).expect("committed delete-layer diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-layer/removes-the-leading-layer-and-keeps-the-overlay: committed diff did not carry before to after");
}
